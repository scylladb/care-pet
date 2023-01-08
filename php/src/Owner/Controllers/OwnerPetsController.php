<?php

namespace App\Owner\Controllers;

use App\Core\Http\BaseController;
use App\Owner\Actions\GetOwnerPets;

final class OwnerPetsController extends BaseController
{
    /**@var GetOwnerPets */
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
