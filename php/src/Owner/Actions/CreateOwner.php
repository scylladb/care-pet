<?php

namespace App\Owner\Actions;

use App\Core\Database\Connector;
use App\Owner\OwnerDTO;
use App\Owner\OwnerFactory;
use App\Owner\OwnerRepository;

final class CreateOwner
{
    /** @var OwnerRepository */
    private $repository;

    public function __construct(OwnerRepository $repository)
    {
        $this->repository = $repository;
    }

    public function handle(): OwnerDTO
    {
        $ownerDTO = OwnerFactory::make();
        $this->repository->create($ownerDTO);

        return $ownerDTO;
    }
}
