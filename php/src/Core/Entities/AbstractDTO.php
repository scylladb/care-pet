<?php

namespace App\Core\Entities;

use Cassandra\Uuid;
use JsonSerializable;

abstract class AbstractDTO implements JsonSerializable
{
    /** @var Uuid $id */
    public $id;

    /** @return \App\Core\Entities\AbstractDTO */
    public static abstract function make(array $payload);

    /** @return string[] */
    public abstract function toDatabase(): array;

    /** @return string[] */
    public function jsonSerialize(): array
    {
        return $this->toDatabase();
    }
}
