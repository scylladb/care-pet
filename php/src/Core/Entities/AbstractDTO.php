<?php

namespace App\Core\Entities;

use Cassandra\Uuid;
use JsonSerializable;

abstract class AbstractDTO implements JsonSerializable
{
    public readonly ?Uuid $id;

    public static abstract function make(array $payload): AbstractDTO;

    /** @return string[] */
    public abstract function toDatabase(): array;

    /** @return string[] */
    public function jsonSerialize(): array
    {
        return $this->toDatabase();
    }
}
