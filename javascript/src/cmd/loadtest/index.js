const { setrlimit } = require('posix');
const parseDuration = require('parse-duration');

const config = require('../../config');
const { getClientWithKeyspace } = require('../../db');
const { createFlock, saveFlock } = require('./flock');
const { work } = require('./worker');
const { start } = require('./pets');

const log = require('../../logger');

async function main() {
  const options = cli(config('loadtest'))
    .parse()
    .opts();

  options.interval = parseDuration(options.interval);

  log.debug(`Configuration = ${JSON.stringify(options)}`);

  log.info('Welcome to the Pets simulator');

  setrlimit('nofile', { soft: 102400, hard: 102400 });

  const client = await getClientWithKeyspace(options);

  const { owners, pets, sensors } = createFlock(options);
  await saveFlock(client, { owners, pets, sensors });

  if (options.writer) {
    await Promise.all(
      new Array(options.workers).fill().map((_, id) => work(client, id, sensors))
    );
  } else {
    await Promise.all(start(client, options.interval, { pets, sensors }));
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
