<?php

namespace App\Sensors\Controllers;

use App\Core\Http\BaseController;
use App\Sensors\Actions\FindSensorsByPetId;

class GetPetSensorsController extends BaseController
{
    /** @var FindSensorsByPetId */
    private $action;

    public function __construct(FindSensorsByPetId $action)
    {
        $this->action = $action;
    }

    public function __invoke(string $petId)
    {
        $this->responseJson($this->action->handle($petId));
    }
}