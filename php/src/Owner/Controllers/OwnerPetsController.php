<?php

namespace App\Owner\Controllers;

use App\Core\Http\BaseController;
use App\Owner\Actions\GetOwnerPets;

final class OwnerPetsController extends BaseController
{
    public function __construct(private readonly GetOwnerPets $action)
    {
    }

    public function __invoke(string $ownerId)
    {
        $this->responseJson($this->action->handle($ownerId));
    }
}
