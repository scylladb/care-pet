const { Owner, Pet, Sensor, randomNumber } = require('../../model');
const { insertQuery } = require('../../db');

const log = require('../../logger');

function createFlock({
  owners: ownersCount,
  pets: petsCount,
  sensors: sensorsCount,
}) {
  const owners = new Array(ownersCount).fill().map(Owner.random);

  log.info('Owners created');

  const pets = new Array(petsCount)
    .fill()
    .map(() =>
      Pet.random(owners[Math.floor(randomNumber(0, owners.length - 1))])
    );

  log.info('Pets created');

  const sensors = pets.reduce((acc, pet) => {
    const sensors = new Array(Math.floor(randomNumber(1, sensorsCount)))
      .fill()
      .map(() => Sensor.random(pet));

    acc[pet.pet_id] = sensors;
    return acc;
  }, {});

  log.info('Sensors created');

  return { owners, pets, sensors };
}

async function saveFlock(client, { owners, pets, sensors }) {
  const batches = [
    [Owner, owners],
    [Pet, pets],
    [Sensor, sensors],
  ].flatMap(([klass, data]) => {
    if (!Array.isArray(data)) {
      data = Object.keys(data).flatMap(key => data[key]);
    }

    return data.map(data =>
      client.execute(insertQuery(klass), data, { prepare: true })
    );
  });

  await Promise.all(batches);
}

module.exports = {
  createFlock,
  saveFlock,
};
