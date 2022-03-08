const path = require('path');
const fs = require('fs');

function readCql(cql) {
  return fs.readFileSync(path.join(__dirname, `${cql}.cql`), 'utf8');
}

const KEYSPACE = readCql('keyspace');
const MIGRATE = readCql('migrate')
  .split(';')
  .map(s => s.trim())
  .filter(s => s);

module.exports = {
  KEYSPACE,
  MIGRATE,
};
