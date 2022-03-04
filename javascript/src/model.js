const cassandra = require('cassandra-driver');
const { Uuid } = cassandra.types;

const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz';

function randomString(length) {
  return new Array(length)
    .fill()
    .reduce(
      str =>
        str + characters.charAt(Math.floor(Math.random() * characters.length)),
      ''
    );
}

function randomNumber(min, max) {
  return Math.random() * (max - min) + min;
}

class Owner {
  static random() {
    const owner = new Owner();
    owner.owner_id = Uuid.random();
    owner.address = 'home';
    owner.name = randomString(8);

    return owner;
  }

  static get tableName() {
    return 'owner';
  }

  static get columns() {
    return ['owner_id', 'address', 'name'];
  }
}

class Pet {
  static random(owner) {
    const pet = new Pet();
    pet.owner_id = owner.owner_id;
    pet.pet_id = Uuid.random();
    pet.age = Math.floor(randomNumber(1, 100));
    pet.weight = randomNumber(5, 10);
    pet.address = owner.address;
    pet.name = randomString(8);

    return pet;
  }

  static get tableName() {
    return 'pet';
  }

  static get columns() {
    return ['owner_id', 'pet_id', 'age', 'weight', 'address', 'name'];
  }
}

class Measure {
  constructor(sensor_id, ts, value) {
    this.sensor_id = sensor_id;
    this.ts = ts;
    this.value = value;
  }

  static get tableName() {
    return 'measurement';
  }

  static get columns() {
    return ['sensor_id', 'ts', 'value'];
  }
}

class SensorAvg {
  constructor(sensor_id, date, hour, value) {
    this.sensor_id = sensor_id;
    this.date = date;
    this.hour = hour;
    this.value = value;
  }

  static get tableName() {
    return 'sensor_avg';
  }

  static get columns() {
    return ['sensor_id', 'date', 'hour', 'value'];
  }
}

class Sensor {
  static random(pet) {
    const sensor = new Sensor();
    sensor.pet_id = pet.pet_id;
    sensor.sensor_id = Uuid.random();
    sensor.type = randomSensorType();

    return sensor;
  }

  static get tableName() {
    return 'sensor';
  }

  static get columns() {
    return ['pet_id', 'sensor_id', 'type'];
  }
}

function randomSensorType() {
  const rnd = Math.floor(randomNumber(0, 3));
  switch (rnd) {
    case 0:
      return 'T';
    case 1:
      return 'P';
    case 2:
      return 'L';
    case 3:
      return 'R';
    default:
      throw new Exception('Unknown sensor type');
  }
}

function randomSensorData(sensor) {
  switch (sensor.type) {
    case 'T':
      return 101 + randomNumber(0, 10) - 4;
    case 'P':
      return 101 + randomNumber(0, 40) - 20;
    case 'L':
      return 35 + randomNumber(0, 5) - 2;
    case 'R':
      return 10 * Math.random();
  }
}

module.exports = {
  Owner,
  Pet,
  Measure,
  SensorAvg,
  Sensor,
  randomSensorData,
  randomNumber,
};
