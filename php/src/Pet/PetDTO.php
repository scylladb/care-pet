<?php

namespace App\Pet;

use App\Core\Entities\AbstractDTO;
use Cassandra\Uuid;
use Cassandra\Varint;

final class PetDTO extends AbstractDTO
{

    public function __construct(
        public readonly Uuid   $ownerId,
        public readonly string   $chipId,
        public readonly string $color,
        public readonly string $breed,
        public readonly string $species,
        public readonly string $gender,
        public readonly int    $age,
        public readonly float  $weight,
        public readonly string $address,
        public readonly string $name,
        public readonly ?Uuid $id = null
    )
    {
    }

    public static function make(array $payload): self
    {
        return new self(
            $payload['owner_id'],
            $payload['chip_id'],
            $payload['color'],
            $payload['breed'],
            $payload['species'],
            $payload['gender'],
            $payload['age'],
            (float) $payload['weight'],
            $payload['address'],
            $payload['name'],
            $payload['pet_id']
        );
    }

    public function toDatabase(): array
    {
        return [
            'pet_id' => $this->id,
            'owner_id' => $this->ownerId,
            'chip_id' => "nah",
            'species' => $this->species,
            'breed' => $this->breed,
            'color' => $this->color,
            'gender' => $this->gender,
            'age' => $this->age,
            //'weight' => new \Cassandra\Decimal($this->weight),
            'address' => $this->address,
            'name' => $this->name,
        ];
    }
}
