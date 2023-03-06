<?php

namespace App\Sensors\Type;

use Faker\Factory;

final class TypeFactory
{
    public static function make(array $fields = []): TypeDTO
    {
        $faker = Factory::create();
        $type = $fields['type'] ?? $faker->randomElement(['T', 'P', 'L', 'R']);

        return new TypeDTO($type);
    }
}
