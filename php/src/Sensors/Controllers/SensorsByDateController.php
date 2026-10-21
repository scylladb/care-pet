<?php

namespace App\Sensors\Controllers;

use App\Core\Http\BaseController;
use App\Sensors\Actions\FindSensorsByDate;
use DateTimeImmutable;
use DateTimeZone;
use Exception;

class SensorsByDateController extends BaseController
{
    public function __construct(private readonly FindSensorsByDate $action)
    {
    }

    public function __invoke(string $sensorId): void
    {
        if (empty($_GET['start_at']) || empty($_GET['end_at'])) {
            $this->responseJson(['error' => 'The start_at and end_at query parameters are required.'], 400);
            return;
        }

        try {
            $utc = new DateTimeZone('UTC');
            $startAt = new DateTimeImmutable($_GET['start_at'], $utc);
            $endAt = new DateTimeImmutable($_GET['end_at'], $utc);
        } catch (Exception) {
            $this->responseJson(['error' => 'The start_at and end_at query parameters must be ISO 8601 dates.'], 400);
            return;
        }

        $this->responseJson($this->action->handle($sensorId, $startAt, $endAt));
    }
}