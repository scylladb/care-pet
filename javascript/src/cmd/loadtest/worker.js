const { randomSensorData, Measure } = require('../../model');
const { insertQuery } = require('../../db');
const { Stats } = require('./stats');

const moment = require('moment');

const log = require('../../logger');

async function work(client, id, sensors) {
  log.debug(`worker # ${id} ready`);

  const prefix = `#${id}`;
  const stats = new Stats();

  while (true) {
    for (let sensorsArray of Object.values(sensors)) {
      for (let sensor of sensorsArray) {
        let measure = new Measure(
          sensor.sensor_id,
          moment().toDate(),
          randomSensorData(sensor)
        );

        const ts = moment();
        try {
          await client.execute(insertQuery(Measure), measure, {
            prepare: true,
          });

          log.debug(`worker # ${id} insert ${JSON.stringify(measure)}`);
        } catch (err) {
          log.error(`worker # ${id} error ${err}`);
        }

        stats.record(ts);
      }
    }

    stats.print(prefix);
  }
}

module.exports = { work };
