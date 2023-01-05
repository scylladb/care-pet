<?php

namespace App\Sensor;

use Cassandra\Rows;

class SensorCollection extends \ArrayIterator
{
    public static function make(Rows $rows): self
    {
        $collection = new self();
        foreach ($rows as $row) {
            $collection->append(SensorDTO::make($row));
        }

        return $collection;
    }

    public function add(SensorDTO $owner): self
    {
        $this->append($owner);

        return $this;
    }
}