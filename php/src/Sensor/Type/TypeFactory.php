<?php

namespace App\Sensor\Type;

use Faker\Factory;

class TypeFactory
{
    public static function make(array $fields = []): TypeDTO
    {
        $faker = Factory::create();
        $type = $fields['type'] ?? $faker->randomElement(['T', 'P', 'L', 'R']);

        return new TypeDTO($type);
    }
}