<?php

namespace App\Sensors\Actions;

use App\Sensors\Sensor\SensorDTO;
use App\Sensors\Sensor\SensorRepository;
use App\Sensors\Sensor\SensorsException;
use Cassandra\Uuid;

class FindSensorById
{

    public function __construct(private readonly SensorRepository $sensorRepository)
    {
    }

    public function handle(Uuid $sensorId): SensorDTO
    {
        $sensor = $this->sensorRepository->getById($sensorId);

        if ($sensor->count() == 0) {
            throw SensorsException::notFound($sensorId->uuid());
        }

        return SensorDTO::make($sensor->first());
    }
}