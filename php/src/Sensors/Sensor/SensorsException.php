<?php

namespace App\Sensors\Sensor;

use Exception;

final class SensorsException extends Exception
{
    public static function invalidSensorType(string $type): self
    {
        return new self(
            sprintf('Sensor type %s is invalid.',  $type),
            422
        );
    }

    public static function noSensorsFound(): self
    {
        return new self('This pet doesn\'t has any sensor.', 404);
    }
}
