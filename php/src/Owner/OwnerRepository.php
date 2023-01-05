<?php

namespace App\Owner;

use App\Core\Database\AbstractRepository;

class OwnerRepository extends AbstractRepository
{
    public $table = 'owner';

    public $primaryKey = 'owner_id';
}