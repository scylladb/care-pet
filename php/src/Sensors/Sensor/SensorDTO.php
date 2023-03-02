<?php

namespace App\Sensors\Sensor;

use App\Core\Entities\AbstractDTO;
use App\Sensors\Type\TypeDTO;
use Cassandra\Uuid;

final class SensorDTO extends AbstractDTO
{

    public function __construct(
        public readonly ?Uuid   $id,
        public readonly Uuid    $petId,
        public readonly TypeDTO $type,
    )
    {
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
