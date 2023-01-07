<?php

namespace App\Pet;

use App\Core\Entities\AbstractDTO;
use Cassandra\Uuid;

final class PetDTO extends AbstractDTO
{
    /** @var \Cassandra\Uuid */
    public $ownerId;

    /** @var string */
    public $chipId;

    /** @var string */
    public $color;

    /** @var string */
    public $breed;

    /** @var string */
    public $species;

    /** @var string */
    public $gender;

    /** @var int */
    public $age;

    /** @var float */
    public $weight;

    /** @var string */
    public $address;

    /** @var string */
    public $name;

    public function __construct(
        Uuid   $ownerId,
        string   $chipId,
        string $color,
        string $breed,
        string $species,
        string $gender,
        int    $age,
        float  $weight,
        string $address,
        string $name,
        Uuid   $id = null
    )
    {
        $this->ownerId = $ownerId;
        $this->chipId = $chipId;
        $this->color = $color;
        $this->breed = $breed;
        $this->species = $species;
        $this->gender = $gender;
        $this->age = $age;
        $this->weight = $weight;
        $this->address = $address;
        $this->name = $name;
        $this->id = $id;
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
            'pet_id' => $this->id->uuid(),
            'owner_id' => $this->ownerId->uuid(),
            'chip_id' => $this->chipId,
            'species' => $this->species,
            'breed' => $this->breed,
            'color' => $this->color,
            'gender' => $this->gender,
            'age' => $this->age,
            'weight' => $this->weight,
            'address' => $this->address,
            'name' => $this->name,
        ];
    }
}
