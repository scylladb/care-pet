const { program, dbConfig } = require('../../config');
const { getClient } = require('../../db');
const cql = require('../../cql');

const log = require('../../logger');

async function main() {
  const opts = dbConfig(program('migrate')).parse().opts();

  log.debug(`Configuration = ${JSON.stringify(opts)}`);

  log.info('Bootstrapping database...');

  const client = await getClient(opts);

  log.info('Creating keyspace...');
  await client.execute(cql.KEYSPACE);
  log.info('Keyspace created');

  log.info('Migrating database...');
  for (const query of cql.MIGRATE) {
    log.debug(`query = ${query}`);
    await client.execute(query);
  }
  log.info('Database migrated');

  return client;
}

main().then(client => client.shutdown());
