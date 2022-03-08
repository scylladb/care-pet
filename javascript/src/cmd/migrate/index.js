const config = require('../../config');
const { getClient } = require('../../db');
const cql = require('../../cql');

const log = require('../../logger');

async function main() {
  const options = config('migrate').parse().opts();

  log.debug(`Configuration = ${JSON.stringify(options)}`);

  log.info('Bootstrapping database...');

  const client = await getClient(options);

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
