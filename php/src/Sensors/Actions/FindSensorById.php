<?php

namespace App\Sensors\Actions;

use App\Sensors\Sensor\SensorDTO;
use App\Sensors\Sensor\SensorRepository;
use App\Sensors\Sensor\SensorsException;

class FindSensorById
{
    /** @var SensorRepository */
    private $sensorRepository;

    public function __construct(SensorRepository $sensorRepository)
    {
        $this->sensorRepository = $sensorRepository;
    }

    public function handle(string $sensorId): SensorDTO
    {
        $sensor = $this->sensorRepository->getById($sensorId);

        if ($sensor->count() == 0) {
            throw SensorsException::notFound($sensorId);
        }

        return SensorDTO::make($sensor->first());
    }
}