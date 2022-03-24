const { NotFoundError } = require('./error');
const { Owner } = require('../model');

const ROUTE = '/api/owner/:owner_id';

function handler(client) {
    return async function handler(req) {
        const { rows } = await client.execute(
            `SELECT * FROM ${Owner.tableName} WHERE owner_id = ?`,
            [req.params.owner_id],
            { prepare: true }
        );

        if (rows.length === 0) {
            throw new NotFoundError(`owner #${req.params.owner_id} not found`);
        }

        return rows[0];
    };
}

module.exports = {
    ROUTE,
    handler,
};
