<?php

namespace App\Sensor;

use Exception;

class SensorsException extends Exception
{
    public static function invalidSensorType(string $type): self
    {
        return new self(
            sprintf('Sensor type %s is invalid.',  $type),
            422
        );
    }
}