<?php

namespace App\Pet;

use App\Core\Entities\AbstractFactory;
use Cassandra\Uuid;
use Faker\Factory;

class PetFactory extends AbstractFactory
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
            (float)$faker->randomNumber(2),
            $faker->address(),
            $faker->firstName(),
            new Uuid($faker->uuid)
        );
    }

    public static function makeMany(int $amount, array $fields = []): PetCollection
    {
        $emptyCollection = array_fill(0, $amount, null);
        $collection = array_map(function () use ($fields) {
            return self::make($fields);
        }, $emptyCollection);

        return new PetCollection($collection);
    }
}