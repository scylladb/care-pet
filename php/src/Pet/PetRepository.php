<?php

namespace App\Pet;

use App\Core\Database\AbstractRepository;

class PetRepository extends AbstractRepository
{
    public $table = 'pet';

    public $primaryKey = 'pet_id';

}