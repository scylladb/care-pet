<?php

namespace App\Owner;

use App\Core\Entities\Collection;
use Cassandra\Rows;

class OwnerCollection extends Collection
{

    public static function make(Rows $ownerList): self
    {
        $collection = new self();
        foreach ($ownerList as $owner) {
            $collection->append(OwnerDTO::make($owner));
        }

        return $collection;
    }

    public function add(OwnerDTO $owner): self
    {
        $this->append($owner);

        return $this;
    }
}