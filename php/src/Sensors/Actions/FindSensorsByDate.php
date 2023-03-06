<?php

namespace App\Sensors\Actions;

use App\Sensors\Sensor\SensorRepository;

class FindSensorsByDate
{
    public function __construct(
        private readonly SensorRepository $sensorRepository,
        private readonly FindSensorById $action
    )
    {
    }


    public function handle(string $sensorId, string $startAt, string $endAt)
    {
        $sensorDTO = $this->action->handle($sensorId);
        $sensorsRange = $this->sensorRepository
            ->getSensorsValuesByDateRange($sensorId, $startAt, $endAt);

        $result = array_map(function ($sensor) {
            return $sensor->value;
        }, (array)$sensorsRange);

        return $result;
    }
}