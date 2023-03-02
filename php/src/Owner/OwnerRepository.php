<?php

namespace App\Owner;

use App\Core\Database\AbstractRepository;

final class OwnerRepository extends AbstractRepository
{
    public string $table = 'owner';

    public string $primaryKey = 'owner_id';

    public array $keys = [
        'owner_id'
    ];
}
