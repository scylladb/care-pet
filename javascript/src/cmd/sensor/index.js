const { delay } = require('../../util');
const { dbConfig, program } = require('../../config');
const { getClientWithKeyspace, insertQuery } = require('../../db');
const {
  Owner,
  Pet,
  Sensor,
  Measure,
  randomNumber,
  randomSensorData,
} = require('../../model');
const parseDuration = require('parse-duration');
const moment = require('moment');

const log = require('../../logger');

async function main() {
  const opts = cli(dbConfig(program('sensor simulator')))
    .parse()
    .opts();

  const bufferInterval = parseDuration(opts.bufferInterval);
  const measure = parseDuration(opts.measure);

  log.debug(`Configuration = ${JSON.stringify(opts)}`);

  log.info(`Welcome to the Pet collar simulator`);

  const client = await getClientWithKeyspace(opts);

  const { owner, pet, sensors } = randomData();

  await saveData(client, owner, pet, sensors);

  await runSensorData(
    client,
    {
      bufferInterval,
      measure,
    },
    sensors
  );

  return client;
}

function cli(program) {
  return program
    .option('-b, --buffer-interval <n>', 'Sensors measurement interval', '1h')
    .option('-m, --measure <n>', 'Buffer to accumulate measures', '60s');
}

function randomData() {
  const owner = Owner.random();
  const pet = Pet.random(owner);
  const sensors = new Array(Math.floor(randomNumber(1, 4)))
    .fill()
    .map(() => Sensor.random(pet));

  return {
    owner,
    pet,
    sensors,
  };
}

async function saveData(client, owner, pet, sensors) {
  await client.execute(insertQuery(Owner), owner, { prepare: true });
  log.info(`New owner # ${owner.owner_id}`);

  await client.execute(insertQuery(Pet), pet, { prepare: true });
  log.info(`New pet # ${pet.pet_id}`);

  for (let sensor of sensors) {
    await client.execute(insertQuery(Sensor), sensor, { prepare: true });
    log.info(`New sensor # ${sensor.sensor_id}`);
  }
}

async function runSensorData(client, { bufferInterval, measure }, sensors) {
  let last = moment();
  while (true) {
    const measures = [];
    while (moment().diff(last) < bufferInterval) {
      await delay(measure);

      measures.push(
        ...sensors.map(sensor => {
          const measure = readSensorData(sensor);
          log.info(
            `sensor # ${sensor.sensor_id} type ${sensor.type} new measure ${
              measure.value
            } ts ${moment(measure.ts).toISOString()}`
          );

          return measure;
        })
      );
    }

    last = last.add(
      measure.valueOf() * (moment().diff(last).valueOf() / measure.valueOf())
    );

    log.info('Pushing data');

    const batch = measures.map(measure => ({
      query: insertQuery(Measure),
      params: measure,
    }));

    await client.batch(batch, { prepare: true });
  }
}

function readSensorData(sensor) {
  return new Measure(sensor.sensor_id, Date.now(), randomSensorData(sensor));
}

main().then(client => client.shutdown());
