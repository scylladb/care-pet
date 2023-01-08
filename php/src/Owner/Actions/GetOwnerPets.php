<?php

namespace App\Owner\Actions;

use App\Owner\OwnerRepository;
use App\Pet\PetCollection;
use App\Pet\PetDTO;
use App\Pet\PetException;
use App\Pet\PetRepository;

final class GetOwnerPets
{
    /** @var PetRepository */
    private $repository;
    /** @var FindOwnerById */
    private $ownerAction;

    public function __construct(PetRepository $repository, FindOwnerById $ownerAction)
    {

        $this->repository = $repository;
        $this->ownerAction = $ownerAction;
    }

    /** @return PetCollection<int, PetDTO> */
    public function handle(string $ownerId): PetCollection
    {
        $owner = $this->ownerAction->handle($ownerId);

        $pets = $this->repository->getByOwnerId($owner->id->uuid());
        if ($pets->count() == 0) {
            throw PetException::noPetsFound();
        }

        return PetCollection::make($pets);
    }
}
