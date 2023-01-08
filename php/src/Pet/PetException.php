<?php

namespace App\Pet;

use Exception;

class PetException extends Exception
{
    public static function noPetsFound(): self
    {
        return new Exception('This person doens\'t owns any pet :/', 404);
    }
}