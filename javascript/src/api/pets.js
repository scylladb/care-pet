const { Pet } = require('../model');

const ROUTE = '/api/owner/:owner_id/pets';

function handler(client) {
  return async function handler(req) {
    const { rows } = await client.execute(
      `SELECT * FROM ${Pet.table} WHERE owner_id = ?`,
      [req.params.owner_id],
      { prepare: true }
    );

    return rows;
  };
}

module.exports = {
  ROUTE,
  handler,
};
