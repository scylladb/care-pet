<?php

namespace App\Pet\Actions;

use App\Pet\Exceptions\PetException;
use App\Pet\PetDTO;
use App\Pet\PetRepository;

class FindPetById
{
    /** @var PetRepository */
    private $petRepository;

    public function __construct(PetRepository $petRepository)
    {
        $this->petRepository = $petRepository;
    }

    public function handle(string $petId): PetDTO
    {
        $pet = $this->petRepository->getById($petId);
        if ($pet->count() == 0) {
            throw PetException::notFound($petId);
        }

        return PetDTO::make($pet->first());
    }
}