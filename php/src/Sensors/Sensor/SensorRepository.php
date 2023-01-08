<?php

namespace App\Sensors\Sensor;

use App\Core\Database\AbstractRepository;
use Cassandra\Rows;

final class SensorRepository extends AbstractRepository
{
    /** @var string */
    public $table = 'sensor';

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
            ->get(5);
    }
}
