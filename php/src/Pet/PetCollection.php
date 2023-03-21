<?php

namespace App\Pet;

use App\Core\Entities\Collection;
use Cassandra\Rows;

final class PetCollection extends Collection
{
    public static function make(Rows $databaseRows): self
    {
        $collection = new self();
        foreach ($databaseRows as $row) {
            $collection->add(PetDTO::make($row));
        }

        return $collection;
    }

    public function add(PetDTO $owner): self
    {
        $this->append($owner);

        return $this;
    }
}
