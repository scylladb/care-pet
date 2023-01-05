<?php

namespace App\Core;

use Cassandra\Uuid;

abstract class AbstractDTO
{
    /** @var Uuid $id */
    public $id;

    public static abstract function make(array $payload);

    public abstract function toDatabase(): array;
}