const { program, dbConfig } = require('./config');
const { getClientWithKeyspace } = require('./db');

const app = require('express')();
const owner = require('./api/owner');
const pets = require('./api/pets');
const sensors = require('./api/sensors');
const measures = require('./api/measures');
const avg = require('./api/avg');

const log = require('./logger');

async function main() {
  const opts = dbConfig(program('care-pet')).parse().opts();

  log.debug(`Configuration = ${JSON.stringify(opts)}`);

  const client = await getClientWithKeyspace(opts);

  app.get(owner.ROUTE, asyncHandler(owner.handler(client)));
  app.get(pets.ROUTE, asyncHandler(pets.handler(client)));
  app.get(sensors.ROUTE, asyncHandler(sensors.handler(client)));
  app.get(measures.ROUTE, asyncHandler(measures.handler(client)));
  app.get(avg.ROUTE, asyncHandler(avg.handler(client)));

  app.listen(8000, () => {
    log.info('Care-pet server started on port 8000!');
  });
}

function asyncHandler(handler) {
  return (req, res, next) =>
    handler(req, res, next)
      .then(data => res.json(data))
      .catch(err => next(err));
}

main();
