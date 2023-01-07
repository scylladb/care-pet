<?php

namespace App\Owner;

use App\Core\Database\AbstractRepository;

class OwnerRepository extends AbstractRepository
{
    /** @var string */
    public $table = 'owner';

    /** @var string */
    public $primaryKey = 'owner_id';

    /** @var array */
    public $keys = [
        'owner_id'
    ];
}