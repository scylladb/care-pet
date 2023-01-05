<?php

namespace App\Owner;

use App\Core\AbstractDTO;
use Cassandra\Uuid;

class OwnerDTO extends AbstractDTO
{
    /** @var Uuid $id */
    public $id;

    /** @var string $name */
    public $name;

    /** @var string $address */
    public $address;

    public function __construct(string $name, string $address, Uuid $id = null)
    {
        $this->id = $id;
        $this->name = $name;
        $this->address = $address;
    }

    public static function make(array $payload): self
    {
        return new self($payload['name'], $payload['address'], $payload['owner_id']);
    }

    public function toDatabase(): array
    {
        return [
            'owner_id' => $this->id,
            'name' => $this->name,
            'address' => $this->address
        ];
    }
}