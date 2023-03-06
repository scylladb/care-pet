<?php

namespace App\Owner\Actions;

use App\Core\Database\Connector;
use App\Owner\OwnerDTO;
use App\Owner\OwnerFactory;
use App\Owner\OwnerRepository;

final class CreateOwner
{

    public function __construct(private readonly OwnerRepository $repository)
    {
    }

    public function handle(): OwnerDTO
    {
        $ownerDTO = OwnerFactory::make();
        $this->repository->create($ownerDTO);

        return $ownerDTO;
    }
}
