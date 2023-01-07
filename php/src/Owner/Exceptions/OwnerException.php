<?php

namespace App\Owner\Exceptions;

use Exception;

class OwnerException extends Exception
{
    public static function notFound(string $id): self
    {
        return new self(sprintf('Owner with id "%s" not found. :/', $id), 404);
    }
}