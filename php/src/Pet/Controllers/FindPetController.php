<?php

namespace App\Pet\Controllers;

use App\Core\Http\BaseController;
use App\Pet\Actions\FindPetById;

class FindPetController extends BaseController
{
    /** @param FindPetById $action */
    private $action;

    public function __construct(FindPetById $action)
    {
        $this->action = $action;
    }

    public function __invoke(string $petId)
    {
        $this->responseJson($this->action->handle($petId));
    }
}