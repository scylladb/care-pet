<?php

namespace App\Pet;

use Cassandra\Rows;

class PetCollection extends \ArrayIterator
{
    public static function make(Rows $databaseRows): self
    {
        $collection = new self();
        foreach ($databaseRows as $row) {
            $collection->append(PetDTO::make($row));
        }

        return $collection;
    }

    public function add(PetDTO $owner): self
    {
        $this->append($owner);

        return $this;
    }
}