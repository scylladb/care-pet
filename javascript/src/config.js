const { Command } = require('commander');

function program(name) {
  const program = new Command();
  program.name(name).version('1.0.0');
  return program;
}

function dbConfig(program) {
  return program
    .option('-h, --hosts [hosts...]', 'Cluster nodes address list', [
      '127.0.0.1',
    ])
    .option(
      '-u, --username <username>',
      'Password based authentication username'
    )
    .option(
      '-p, --password <password>',
      'Password based authentication password'
    );
}

module.exports = {
  program,
  dbConfig,
};
