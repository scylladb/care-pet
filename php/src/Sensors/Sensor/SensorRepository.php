<?php

namespace App\Sensors\Sensor;

use App\Core\Database\AbstractRepository;
use App\Core\Database\Connector;
use Cassandra\Date;
use Cassandra\FutureRows;
use Cassandra\Rows;
use Cassandra\Timestamp;
use Cassandra\Uuid;

final class SensorRepository extends AbstractRepository
{
    public string $table = 'sensor';

    public string $primaryKey = 'sensor_id';

    public array $keys = [
        'sensor_id',
        'pet_id'
    ];

    public function getSensorsByPetId(Uuid $petId): Rows
    {
        return $this->connection
            ->prepare('SELECT * FROM sensor WHERE pet_id = ?')
            ->execute([$petId])
            ->get(Connector::BASE_TIMEOUT);
    }

    public function getSensorsValuesByDateRange(Uuid $sensorId, Timestamp $startAt, Timestamp $endAt): Rows
    {
        return $this->connection
            ->prepare('SELECT ts, value FROM measurement WHERE sensor_id = ? AND ts >= ? AND ts <= ?')
            ->execute([$sensorId, $startAt, $endAt])
            ->get(Connector::BASE_TIMEOUT);
    }

    public function createMeasurement(Uuid $sensorId, Timestamp $ts, float $value): FutureRows
    {
        return $this->connection
            ->prepare('INSERT INTO measurement (sensor_id, ts, value) VALUES (?, ?, ?)')
            ->execute([$sensorId, $ts, new \Cassandra\Float($value)]);
    }

    public function getSensorAvgByDate(Uuid $sensorId, Date $date): Rows
    {
        return $this->connection
            ->prepare('SELECT hour, value FROM sensor_avg WHERE sensor_id = ? AND date = ?')
            ->execute([$sensorId, $date])
            ->get(Connector::BASE_TIMEOUT);
    }

    public function createSensorAvg(Uuid $sensorId, Date $date, int $hour, float $value): void
    {
        $this->connection
            ->prepare('INSERT INTO sensor_avg (sensor_id, date, hour, value) VALUES (?, ?, ?, ?)')
            ->execute([$sensorId, $date, $hour, new \Cassandra\Float($value)])
            ->get(Connector::BASE_TIMEOUT);
    }
}
