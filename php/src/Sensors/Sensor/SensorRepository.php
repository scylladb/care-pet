<?php

namespace App\Sensors\Sensor;

use App\Core\Database\AbstractRepository;
use App\Core\Database\Connector;
use Cassandra\Rows;

final class SensorRepository extends AbstractRepository
{
    /** @var string */
    public string $table = 'sensor';

    /** @var string */
    public $primaryKey = 'sensor_id';

    /**@var array */
    public $keys = [
        'sensor_id',
        'pet_id'
    ];

    public function getSensorsByPetId(string $petId): Rows
    {
        $query = sprintf('SELECT * FROM %s where pet_id = %s', $this->table, $petId);

        return $this->connection
            ->prepare($query)
            ->execute()
            ->get(Connector::BASE_TIMEOUT);
    }

    public function getSensorsValuesByDateRange(string $sensorId, string $startAt, string $endAt): Rows
    {
        $query = sprintf(
            "SELECT value FROM %s WHERE sensor_id = %s AND ts >= %s AND ts <= %s",
            $this->table,
            $sensorId,
            $startAt,
            $endAt
        );

        return $this->connection
            ->prepare($query)
            ->execute()
            ->get(Connector::BASE_TIMEOUT);
    }
}
