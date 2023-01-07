<?php

namespace App\Sensor\Type;

use App\Core\Entities\AbstractDTO;
use App\Sensor\SensorsException;

class TypeDTO extends AbstractDTO
{
    private $validTypes = ['T', 'P', 'L', 'R'];

    /**
     * @var string $name
     */
    public $name;

    /**
     * @throws SensorsException
     */
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