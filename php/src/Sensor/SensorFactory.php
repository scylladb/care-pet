<?php

namespace App\Sensor;

use App\Core\Entities\AbstractFactory;
use App\Sensor\Type\TypeFactory;
use Cassandra\Uuid;
use Faker\Factory;
use Faker\Generator;

class SensorFactory extends AbstractFactory
{
    public function __construct()
    {
    }

    public static function make(array $fields = []): SensorDTO
    {
        $faker = Factory::create();

        return new SensorDTO(
            $fields['owner_id'] ?? new Uuid($faker->uuid()),
            $fields['pet_id'] ?? new Uuid($faker->uuid()),
            $fields['type'] ?? TypeFactory::make()
        );
    }

    public static function makeMany(int $amount, array $fields = []): SensorCollection
    {
        $emptyCollection = array_fill(0, $amount, null);
        $collection = array_map(function () use ($fields) {
            return self::make($fields);
        }, $emptyCollection);
        return new SensorCollection($collection);
    }

}