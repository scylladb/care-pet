const { Sensor } = require('../model');

const ROUTE = '/api/pet/:pet_id/sensors';

function handler(client) {
  return async function handler(req) {
    const { rows } = await client.execute(
      `SELECT * FROM ${Sensor.table} WHERE pet_id = ?`,
      [req.params.pet_id],
      { prepare: true }
    );

    return rows;
  };
}

module.exports = {
  ROUTE,
  handler,
};
