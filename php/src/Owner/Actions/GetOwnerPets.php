<?php

namespace App\Owner\Actions;

use App\Pet\Exceptions\PetException;
use App\Pet\PetCollection;
use App\Pet\PetDTO;
use App\Pet\PetRepository;

final class GetOwnerPets
{
    public function __construct(
        private readonly PetRepository $repository,
        private readonly FindOwnerById $ownerAction
    )
    {
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
