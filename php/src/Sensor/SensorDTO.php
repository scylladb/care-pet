<?php

namespace App\Sensor;

use App\Core\Entities\AbstractDTO;
use App\Sensor\Type\TypeDTO;
use Cassandra\Uuid;

class SensorDTO extends AbstractDTO
{
    /*** @var Uuid $id */
    public $id;

    /** @var Uuid $petId */
    public $petId;

    /** @var TypeDTO $type */
    public $type;

    public function __construct(Uuid $id, Uuid $petId, TypeDTO $type)
    {
        $this->id = $id;
        $this->petId = $petId;
        $this->type = $type;
    }

    public function toDatabase(): array
    {
        return [
            'sensor_id' => $this->id->uuid(),
            'pet_id' => $this->petId->uuid(),
            'type' => $this->type->name
        ];
    }

    public static function make(array $payload = []): self
    {
        return new self(
            $payload['pet_id'],
            $payload['sensor_id'],
            TypeDTO::make(['name' => $payload['type']])
        );
    }
}