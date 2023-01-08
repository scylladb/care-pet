<?php

namespace App\Sensors\Actions;

use App\Sensors\Sensor\SensorRepository;

class FindSensorsByDate
{
    /** @var SensorRepository */
    private $sensorRepository;

    /** @var FindSensorById */
    private $action;

    public function __construct(SensorRepository $sensorRepository, FindSensorById $action)
    {
        $this->sensorRepository = $sensorRepository;
        $this->action = $action;
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