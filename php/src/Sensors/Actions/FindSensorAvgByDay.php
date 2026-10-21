<?php

namespace App\Sensors\Actions;

use App\Sensors\Sensor\SensorRepository;
use Cassandra\Date;
use Cassandra\Timestamp;
use Cassandra\Uuid;
use DateTimeImmutable;
use DateTimeZone;

/**
 * Reads the hourly averages of a sensor for one day from the sensor_avg table.
 * If hours are missing, it aggregates them from the measurement table
 * and saves the completed hours to sensor_avg.
 */
class FindSensorAvgByDay
{
    public function __construct(
        private readonly SensorRepository $sensorRepository,
        private readonly FindSensorById $action
    )
    {
    }

    /** @return float[] The averages ordered by hour. */
    public function handle(Uuid $sensorId, DateTimeImmutable $day): array
    {
        $sensorDTO = $this->action->handle($sensorId);
        $date = Date::fromDateTime($day);

        $averages = [];
        foreach ($this->sensorRepository->getSensorAvgByDate($sensorDTO->id, $date) as $row) {
            $averages[$row['hour']] = $row['value']->toDouble();
        }

        // The current hour of today is not complete. Do not aggregate later hours.
        $now = new DateTimeImmutable('now', new DateTimeZone('UTC'));
        $isToday = $day->format('Y-m-d') === $now->format('Y-m-d');
        $lastHour = $isToday ? (int)$now->format('G') : 23;

        if (count($averages) < $lastHour + 1) {
            $measurements = $this->sensorRepository->getSensorsValuesByDateRange(
                $sensorDTO->id,
                Timestamp::fromDateTime($day),
                Timestamp::fromDateTime($day->setTime(23, 59, 59, 999999))
            );

            $sums = [];
            $counts = [];
            foreach ($measurements as $row) {
                $hour = (int)gmdate('G', $row['ts']->time());
                $sums[$hour] = ($sums[$hour] ?? 0) + $row['value']->toDouble();
                $counts[$hour] = ($counts[$hour] ?? 0) + 1;
            }

            for ($hour = 0; $hour <= $lastHour; $hour++) {
                if (isset($averages[$hour])) {
                    continue;
                }

                $averages[$hour] = isset($counts[$hour]) ? $sums[$hour] / $counts[$hour] : 0.0;

                if (!$isToday || $hour < $lastHour) {
                    $this->sensorRepository->createSensorAvg($sensorDTO->id, $date, $hour, $averages[$hour]);
                }
            }
        }

        ksort($averages);

        return array_map(fn(float $value) => round($value, 2), array_values($averages));
    }
}
