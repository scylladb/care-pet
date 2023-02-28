<?php

namespace App\Pet;

use App\Core\Database\AbstractRepository;
use App\Core\Database\Connector;
use Cassandra\Rows;

final class PetRepository extends AbstractRepository
{
    /** @var string */
    public string $table = 'pet';

    /** @var string */
    public $primaryKey = 'pet_id';

    /** @var string[] */
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
            ->get(Connector::BASE_TIMEOUT);
    }
}
