const cassandra = require('cassandra-driver');

const KEYSPACE = 'carepet';

async function getClient(config, keyspace) {
  const client = new cassandra.Client({
    contactPoints: config.hosts,
    authProvider: new cassandra.auth.PlainTextAuthProvider(
      config.username,
      config.password
    ),
    localDataCenter: 'datacenter1',
    keyspace,
  });

  await client.connect();

  return client;
}

async function getClientWithKeyspace(config) {
  return getClient(config, KEYSPACE);
}

function insertQuery(klass) {
  const table = klass.table;
  const values = klass.columns.map(() => '?').join(', ');
  return `INSERT INTO ${table} (${fields(klass)}) VALUES (${values})`;
}

function fields(klass) {
  return klass.columns.join(', ');
}

module.exports = {
  getClient,
  getClientWithKeyspace,
  insertQuery,
  fields,
};
