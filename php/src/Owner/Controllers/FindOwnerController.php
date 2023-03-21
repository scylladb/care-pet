<?php

namespace App\Owner\Controllers;

use App\Core\Database\Connector;
use App\Core\Http\BaseController;
use App\Owner\Actions\CreateOwner;
use App\Owner\Actions\FindOwnerById;

final class FindOwnerController extends BaseController
{
    public function __construct(private readonly FindOwnerById $action)
    {
    }

    public function __invoke(string $ownerId)
    {
        $this->responseJson($this->action->handle($ownerId));
    }
}
