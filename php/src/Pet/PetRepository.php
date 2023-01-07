<?php

namespace App\Pet;

use App\Core\Database\AbstractRepository;
use Cassandra\Rows;

class PetRepository extends AbstractRepository
{
    public $table = 'pet';

    public $primaryKey = 'pet_id';

    public $keys = [
        'pet_id',
        'owner_id'
    ];

    public function getByOwnerId(string $ownerId): Rows
    {
        $query = sprintf('SELECT * FROM %s where owner_id = %s', $this->table, $ownerId);

        return $this->connection
            ->prepare($query)
            ->execute()
            ->get(5);
    }
}