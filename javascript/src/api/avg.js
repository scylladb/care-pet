const moment = require('moment');
const { SensorAvg, Measure } = require('../model');
const { insertQuery } = require('../db');

const ROUTE = '/api/sensor/:sensor_id/values/day/:date';

function handler(client) {
    return async function handler(req) {
        const sensor_id = req.params.sensor_id;
        const date = moment(req.params.date, 'YYYY-MM-DD');

        if (date > moment()) {
            throw new Error('day cannot be in the future');
        }

        let { rows } = await client.execute(
            `SELECT * FROM ${SensorAvg.tableName} WHERE sensor_id = ? AND date = ?`,
            [sensor_id, date.toDate()],
            { prepare: true }
        );

        if (rows.length < 24) {
            rows = await aggregate(client, sensor_id, date, rows);
        }

        return rows.map((row) => row.value);
    };
}

async function aggregate(client, id, date, avg) {
    const now = moment();

    const startDate = date
        .clone()
        .set({ hour: avg.length, minute: 0, second: 0, millisecond: 0 });

    const endDate = date
        .clone()
        .set({ hour: 23, minute: 59, second: 59, millisecond: 0 });

    const data = await loadData(client, id, startDate, endDate);
    const grouped = groupBy(avg, data, avg.length, date, now);
    await saveAggregate(
        client,
        id,
        [...grouped],
        avg.length,
        startDate,
        date,
        now
    );
    return grouped;
}

async function loadData(client, id, startDate, endDate) {
    const { rows } = await client.execute(
        `SELECT ts, value FROM ${Measure.table} WHERE sensor_id = ? AND ts >= ? AND ts <= ?`,
        [id, startDate.toDate(), endDate.toDate()]
    );

    return rows;
}

function groupBy(avg, data, startHour, date, now) {
    const sameDay = date.isSame(now, 'day');
    const last = now.hour();

    const ag = data.reduce((acc, { ts, value }) => {
        const hour = moment(ts).hour();
        if (!acc[hour]) {
            acc[hour] = { value: 0, total: 0 };
        }

        acc[hour].value += value;
        acc[hour].total += 1;
        return acc;
    }, {});

    for (let hour = startHour; hour <= 24; hour++) {
        if (!sameDay || hour < last) {
            if (!ag[hour]) {
                ag[hour] = { value: 0, total: 0 };
            }
        }
    }

    const result = avg.concat(
        Object.keys(ag).map((hour) => {
            const { value, total } = ag[hour];
            const sa = { hour, value: 0 };
            if (total > 0) {
                sa.value = value / total;
            }

            return sa;
        })
    );

    result.sort((a, b) => a.hour - b.hour);
    return result;
}

async function saveAggregate(client, id, avg, prevAvgSize, date, now) {
    const sameDay = date.isSame(now, 'day');
    const current = now.hour();

    avg.splice(0, prevAvgSize);
    const toInsert = avg.filter((avg) => sameDay || avg.hour < current);

    for (let { value, hour } of toInsert) {
        await client.execute(
            insertQuery(SensorAvg),
            [id, date.toDate(), hour, value],
            { prepare: true }
        );
    }

    return toInsert;
}

module.exports = {
    ROUTE,
    handler,
};
