<?php

namespace App\Sensors\Controllers;

use App\Core\Http\BaseController;
use App\Sensors\Actions\FindSensorAvgByDay;
use DateTimeImmutable;
use DateTimeZone;

class SensorAvgByDayController extends BaseController
{
    public function __construct(private readonly FindSensorAvgByDay $action)
    {
    }

    public function __invoke(string $sensorId): void
    {
        $utc = new DateTimeZone('UTC');
        $day = DateTimeImmutable::createFromFormat('!Y-m-d', $_GET['date'] ?? '', $utc);

        if ($day === false || $day->format('Y-m-d') !== $_GET['date']) {
            $this->responseJson(['error' => 'The date query parameter is required. Use the format YYYY-MM-DD.'], 400);
            return;
        }

        if ($day > new DateTimeImmutable('now', $utc)) {
            $this->responseJson(['error' => 'The date must not be in the future.'], 400);
            return;
        }

        $this->responseJson($this->action->handle($sensorId, $day));
    }
}
