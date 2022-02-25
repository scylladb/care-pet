const { randomSensorData, Measure } = require('../../model');
const { insertQuery } = require('../../db');
const { delay } = require('../../util');

const moment = require('moment');

const log = require('../../logger');

function start(client, interval, { pets, sensors }) {
  return pets.map(async pet => {
    const petSensors = sensors[pet.pet_id];

    log.debug(`pet # ${pet.pet_id} ready`);

    while (true) {
      for (let sensor of petSensors) {
        let measure = new Measure(
          sensor.sensor_id,
          moment().toDate(),
          randomSensorData(sensor)
        );

        try {
          await client.execute(insertQuery(Measure), measure, {
            prepare: true,
          });
          log.debug(`pet insert ${JSON.stringify(measure)}`);
        } catch (err) {
          log.error(`pet error ${err}`);
        }
      }

      await delay(interval);
    }
  });
}

module.exports = { start };
