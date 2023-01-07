<?php

namespace App\Owner\Controllers;

use App\Core\Http\BaseController;
use App\Owner\Actions\GetOwnerPets;

class OwnerPetsController extends BaseController
{
    /**@var \App\Owner\Actions\GetOwnerPets */
    private $action;

    public function __construct(GetOwnerPets $action)
    {
        $this->action = $action;
    }

    public function __invoke(string $ownerId)
    {
        $this->responseJson($this->action->handle($ownerId));
    }
}