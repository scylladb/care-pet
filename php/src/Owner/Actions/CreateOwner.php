<?php

namespace App\Owner\Actions;

use App\Core\Database\Connector;
use App\Owner\OwnerDTO;
use App\Owner\OwnerFactory;
use App\Owner\OwnerRepository;

class CreateOwner
{
    /**
     * @var \App\Owner\OwnerRepository
     */
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