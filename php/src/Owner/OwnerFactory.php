<?php

namespace App\Owner;


use App\Core\Entities\AbstractFactory;
use Cassandra\Uuid;
use Faker\Factory;

class OwnerFactory extends AbstractFactory
{
    public static function make(array $fields = []): OwnerDTO
    {
        $faker = Factory::create();

        return new OwnerDTO(
            $faker->name(),
            $faker->address(),
            new Uuid($faker->uuid())
        );
    }

    public static function makeMany(int $amount, array $fields = []): OwnerCollection
    {
        $collection = array_fill(0, $amount, self::make($fields));
        return new OwnerCollection($collection);
    }
}