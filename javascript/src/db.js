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

function insertQuery(table) {
  const tableName = table.tableName;
  const values = table.columns.map(() => '?').join(', ');
  return `INSERT INTO ${tableName} (${fields(table)}) VALUES (${values})`;
}

function fields(table) {
  return table.columns.join(', ');
}

module.exports = {
  getClient,
  getClientWithKeyspace,
  insertQuery,
  fields,
};
