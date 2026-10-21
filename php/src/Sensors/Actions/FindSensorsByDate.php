<?php

namespace App\Sensors\Actions;

use App\Sensors\Sensor\SensorRepository;
use Cassandra\Timestamp;
use DateTimeImmutable;

class FindSensorsByDate
{
    public function __construct(
        private readonly SensorRepository $sensorRepository,
        private readonly FindSensorById $action
    )
    {
    }

    /** @return float[] */
    public function handle(string $sensorId, DateTimeImmutable $startAt, DateTimeImmutable $endAt): array
    {
        $sensorDTO = $this->action->handle($sensorId);
        $sensorsRange = $this->sensorRepository->getSensorsValuesByDateRange(
            $sensorDTO->id,
            Timestamp::fromDateTime($startAt),
            Timestamp::fromDateTime($endAt)
        );

        $result = [];
        foreach ($sensorsRange as $row) {
            $result[] = round($row['value']->toDouble(), 2);
        }

        return $result;
    }
}