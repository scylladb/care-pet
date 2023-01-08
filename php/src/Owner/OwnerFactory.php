<?php

namespace App\Owner;


use App\Core\Entities\AbstractFactory;
use Cassandra\Uuid;
use Faker\Factory;

final class OwnerFactory extends AbstractFactory
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
        $emptyCollection = array_fill(0, $amount, null);
        $collection = array_map(function () use ($fields) {
            return self::make($fields);
        }, $emptyCollection);

        return new OwnerCollection($collection);
    }
}
