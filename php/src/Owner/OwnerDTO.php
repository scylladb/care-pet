<?php

namespace App\Owner;

use App\Core\Entities\AbstractDTO;
use Cassandra\Uuid;

final class OwnerDTO extends AbstractDTO
{
    public function __construct(
        public readonly string $name,
        public readonly string $address,
        public readonly ?Uuid  $id = null
    )
    {
    }

    public static function make(array $payload): self
    {
        return new self(
            name: $payload['name'],
            address: $payload['address'],
            id: $payload['owner_id']
        );
    }

    public function toDatabase(): array
    {
        return [
            'owner_id' => $this->id->uuid(),
            'name' => $this->name,
            'address' => $this->address
        ];
    }
}
