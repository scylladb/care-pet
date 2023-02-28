<?php

namespace App\Owner;

use App\Core\Database\AbstractRepository;

final class OwnerRepository extends AbstractRepository
{
    /** @var string */
    public string $table = 'owner';

    /** @var string */
    public $primaryKey = 'owner_id';

    /** @var string[] */
    public $keys = [
        'owner_id'
    ];
}
