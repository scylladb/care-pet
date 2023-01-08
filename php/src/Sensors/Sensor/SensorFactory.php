<?php

namespace App\Sensors\Sensor;

use App\Core\Entities\AbstractFactory;
use App\Sensors\Type\TypeFactory;
use Cassandra\Uuid;
use Faker\Factory;

final class SensorFactory extends AbstractFactory
{
    public function __construct()
    {
    }

    public static function make(array $fields = []): SensorDTO
    {
        $faker = Factory::create();

        return new SensorDTO(
            new Uuid($faker->uuid()),
            $fields['pet_id'] ?? new Uuid($faker->uuid()),
            $fields['type'] ?? TypeFactory::make()
        );
    }

    /**
     * @param int $amount
     * @param array $fields
     * @return SensorCollection<int, \App\Sensors\Sensor\SensorDTO>
     */
    public static function makeMany(int $amount, array $fields = []): SensorCollection
    {
        $emptyCollection = array_fill(0, $amount, null);
        $collection = array_map(function () use ($fields) {
            return self::make($fields);
        }, $emptyCollection);

        return new SensorCollection($collection);
    }
}
