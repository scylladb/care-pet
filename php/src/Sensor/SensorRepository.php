<?php

namespace App\Sensor;

use App\Core\Database\AbstractRepository;

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
}
