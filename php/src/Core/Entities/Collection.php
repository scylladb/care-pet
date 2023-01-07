<?php

namespace App\Core\Entities;

use ArrayIterator;
use Closure;
use JsonSerializable;

abstract class Collection extends ArrayIterator implements JsonSerializable
{

    public function jsonSerialize(): array
    {
        return [
            'data' => $this->getArrayCopy()
        ];
    }

    public function each(Closure $closure): self
    {
        foreach ($this->getArrayCopy() as $item) {
            $closure($item);
        }

        return $this;
    }
}