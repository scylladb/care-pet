<?php

namespace App\Core\Commands\Base;

use Exception;

class CommandException extends Exception
{
    public static function notFound(string $commandPrefix): self
    {
        return new self(sprintf('Command %s not found.', $commandPrefix));
    }
}