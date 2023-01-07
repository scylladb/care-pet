<?php

namespace App\Owner\Actions;

use App\Owner\OwnerRepository;
use App\Pet\PetCollection;
use App\Pet\PetRepository;

class GetOwnerPets
{
    /** @var \App\Pet\PetRepository */
    private $repository;
    /** @var \App\Owner\Actions\FindOwnerById */
    private $ownerAction;

    public function __construct(PetRepository $repository, FindOwnerById $ownerAction)
    {

        $this->repository = $repository;
        $this->ownerAction = $ownerAction;
    }

    public function handle(string $ownerId)
    {
        $owner = $this->ownerAction->handle($ownerId);
        return PetCollection::make(
            $this->repository->getByOwnerId($owner->id->uuid())
        );
    }
}