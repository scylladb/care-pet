<?php

namespace App\Owner;


use Cassandra\Uuid;
use Faker\Factory;

class OwnerFactory
{
    public static function make(): OwnerDTO
    {
        $faker = Factory::create();

        return new OwnerDTO(
            $faker->name(),
            $faker->address(),
            new Uuid($faker->uuid())
        );
    }
}