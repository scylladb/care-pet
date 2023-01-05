<?php

namespace App\Pet;

use Cassandra\Uuid;
use Faker\Factory;

class PetFactory
{
    public static function make(array $fields = []): PetDTO
    {
        $faker = Factory::create();
        return new PetDTO(
            $fields['owner_id'] ?: new Uuid($faker->uuid()),
            $faker->uuid(),
            $faker->colorName(),
            $faker->word(),
            $faker->word(),
            $faker->randomElement(['male', 'female']),
            $faker->randomNumber(2),
            (float) $faker->randomNumber(2),
            $faker->address(),
            $faker->name(),
            new Uuid($faker->uuid())
        );
    }
}