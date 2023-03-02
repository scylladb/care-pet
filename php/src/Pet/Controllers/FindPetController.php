<?php

namespace App\Pet\Controllers;

use App\Core\Http\BaseController;
use App\Pet\Actions\FindPetById;

class FindPetController extends BaseController
{
    public function __construct(private readonly FindPetById $action)
    {
    }

    public function __invoke(string $petId)
    {
        $this->responseJson($this->action->handle($petId));
    }
}