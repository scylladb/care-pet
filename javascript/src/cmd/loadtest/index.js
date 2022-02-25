const { setrlimit } = require('posix');
const parseDuration = require('parse-duration');

const { dbConfig, program } = require('../../config');
const { getClientWithKeyspace } = require('../../db');
const { createFlock, saveFlock } = require('./flock');
const { work } = require('./worker');
const { start } = require('./pets');

const log = require('../../logger');

async function main() {
  const opts = cli(dbConfig(program('loadtest')))
    .parse()
    .opts();

  opts.interval = parseDuration(opts.interval);

  log.debug(`Configuration = ${JSON.stringify(opts)}`);

  log.info('Welcome to the Pets simulator');

  setrlimit('nofile', { soft: 102400, hard: 102400 });

  const client = await getClientWithKeyspace(opts);

  const { owners, pets, sensors } = createFlock(opts);
  await saveFlock(client, { owners, pets, sensors });

  if (opts.writer) {
    await Promise.all(
      new Array(opts.workers).fill().map((_, id) => work(client, id, sensors))
    );
  } else {
    await Promise.all(start(client, opts.interval, { pets, sensors }));
  }

  return client;
}

function cli(program) {
  return program
    .option('--owners <n>', 'number of the startPets owners', 100)
    .option('--pets <n>', 'number of startPets to simulate', 100)
    .option('--sensors <n>', 'number of sensors per pet', 4)
    .option('--interval <n>', 'an interval between sensors measurements', '1s')
    .option('--writer', 'just write random data')
    .option(
      '--workers',
      'number of parallel writers: Default: num of CPUs',
      require('os').cpus().length
    );
}

main().then(client => client.shutdown());
