<?php

namespace App\Core\Entities;

use Cassandra\Uuid;
use JsonSerializable;

abstract class AbstractDTO implements JsonSerializable
{
    /** @var Uuid $id */
    public $id;

    public static abstract function make(array $payload);

    public abstract function toDatabase(): array;

    public function jsonSerialize(): array
    {
        return $this->toDatabase();
    }
}