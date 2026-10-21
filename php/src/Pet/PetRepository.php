<?php

namespace App\Pet;

use App\Core\Database\AbstractRepository;
use App\Core\Database\Connector;
use Cassandra\Rows;
use Cassandra\Uuid;

final class PetRepository extends AbstractRepository
{
    public string $table = 'pet';

    public string $primaryKey = 'pet_id';

    public array $keys = [
        'pet_id',
        'owner_id'
    ];

    public function getByOwnerId(Uuid $ownerId): Rows
    {
        return $this->connection
            ->prepare('SELECT * FROM pet WHERE owner_id = ?')
            ->execute([$ownerId])
            ->get(Connector::BASE_TIMEOUT);
    }
}
