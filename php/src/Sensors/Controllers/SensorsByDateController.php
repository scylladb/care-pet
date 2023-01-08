<?php

namespace App\Sensors\Controllers;

use App\Core\Http\BaseController;
use App\Sensors\Actions\FindSensorsByDate;

class SensorsByDateController extends BaseController
{
    /** @var FindSensorsByDate */
    private $action;

    public function __construct(FindSensorsByDate $action)
    {
        $this->action = $action;
    }

    public function handle(string $sensorId): void
    {
        if (!isset($_GET['start_at']) || !isset($_GET['end_at'])) {

        }

        $this->responseJson($this->action->handle(
            $sensorId.
            $_GET['start_at'],
            $_GET['end_at']
        ));
    }
}