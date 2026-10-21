<?php

namespace App\Pet\Actions;

use App\Pet\Exceptions\PetException;
use App\Pet\PetDTO;
use App\Pet\PetRepository;
use Cassandra\Uuid;

class FindPetById
{
    public function __construct(
        private readonly PetRepository $petRepository
    )
    {
    }

    public function handle(Uuid $petId): PetDTO
    {
        $pet = $this->petRepository->getById($petId);
        if ($pet->count() == 0) {
            throw PetException::notFound($petId->uuid());
        }

        return PetDTO::make($pet->first());
    }
}