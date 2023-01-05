<?php

namespace App\Sensor;

use App\Core\Database\AbstractRepository;

class SensorRepository extends AbstractRepository
{
    public $table = 'sensor';

    public $primaryKey = 'sensor_id';
}