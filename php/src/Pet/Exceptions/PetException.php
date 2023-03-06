<?php

namespace App\Pet\Exceptions;

use Exception;

class PetException extends Exception
{
    public static function noPetsFound(): self
    {
        return new self('This person doens\'t owns any pet :/', 404);
    }

    public static function notFound(string $petId): self
    {
        return new self(sprintf('Pet with id %s not found :/', $petId), 404);
    }
}