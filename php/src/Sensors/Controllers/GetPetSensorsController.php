<?php

namespace App\Sensors\Controllers;

use App\Core\Http\BaseController;
use App\Sensors\Actions\FindSensorsByPetId;

class GetPetSensorsController extends BaseController
{
    public function __construct(
        private readonly FindSensorsByPetId $action
    )
    {
    }

    public function __invoke(string $petId)
    {
        $this->responseJson($this->action->handle($petId));
    }
}