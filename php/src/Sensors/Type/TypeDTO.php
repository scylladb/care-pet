<?php

namespace App\Sensors\Type;

use App\Core\Entities\AbstractDTO;
use App\Sensors\Sensor\SensorsException;

final class TypeDTO extends AbstractDTO
{
    private array $validTypes = ['T', 'P', 'L', 'R'];

    public string $name;

    /** @throws SensorsException */
    public function __construct(string $name)
    {
        if (!in_array($name, $this->validTypes)) {
            throw SensorsException::invalidSensorType($name);
        }

        $this->name = $name;
    }

    public static function make(array $payload): self
    {
        return new self($payload['name']);
    }

    public function toDatabase(): array
    {
        return [];
    }
}
