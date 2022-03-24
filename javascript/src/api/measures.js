const moment = require('moment');
const { Measure } = require('../model');

const ROUTE = '/api/sensor/:sensor_id/values';

function handler(client) {
    return async function handler(req) {
        const sensor_id = req.params.sensor_id;
        const from = moment(req.query.from).toDate();
        const to = moment(req.query.to).toDate();

        const { rows } = await client.execute(
            `SELECT value FROM ${Measure.tableName} WHERE sensor_id = ? AND ts >= ? AND ts <= ?`,
            [sensor_id, from, to],
            { prepare: true }
        );

        return rows.map((row) => row.value);
    };
}

module.exports = {
    ROUTE,
    handler,
};
