<?php

namespace App\Core\Commands;

use App\Core\Commands\Base\AbstractCommand;
use App\Core\Database\Connector;
use App\Owner\OwnerFactory;
use App\Owner\OwnerRepository;
use App\Pet\PetFactory;
use App\Pet\PetRepository;
use App\Sensors\Sensor\SensorDTO;
use App\Sensors\Sensor\SensorFactory;
use App\Sensors\Sensor\SensorRepository;
use Cassandra\Timestamp;
use Closure;

final class SimulateCommand extends AbstractCommand
{

    public function __construct(
        private readonly OwnerRepository $ownerRepository,
        private readonly PetRepository    $petRepository,
        private readonly SensorRepository $sensorRepository
    )
    {
    }

    const AMOUNT_BASE = 50000;

    // Each sensor gets one measurement every 15 minutes for the last 24 hours.
    const MEASUREMENT_INTERVAL = 900;
    const MEASUREMENT_PERIOD = 86400;

    public function __invoke(array $args = []): int
    {
        $this->info('Starting Sensor simulator...');
        foreach (range(0, self::AMOUNT_BASE) as $i) {
            $this->info("Batch: " . $i);
            [$ownerDTO, $petsDTO] = $this->generateFakeData();

            $this->ownerRepository->create($ownerDTO);
            $this->info(sprintf('Owner %s', $ownerDTO->id));

            $petsDTO->each(function ($petDTO) {
                $this->info(sprintf('Pet: %s | Owner %s', $petDTO->id->uuid(), $petDTO->ownerId));
                $this->petRepository->create($petDTO);

                SensorFactory::makeMany(5, ['pet_id' => $petDTO->id])
                    ->each($this->handleSensors());
            });
        }
        $this->info('Done :D');

        return self::SUCCESS;
    }

    private function generateFakeData(): array
    {
        $ownerDTO = OwnerFactory::make();
        $petsDTO = PetFactory::makeMany(5, ['owner_id' => $ownerDTO->id]);

        return [$ownerDTO, $petsDTO];
    }

    private function handleSensors(): Closure
    {
        return function (SensorDTO $sensorDTO) {
            $this->sensorRepository->create($sensorDTO);
            $this->info(sprintf(
                'Sensor: %s (%s) | Pet %s',
                $sensorDTO->id,
                $sensorDTO->type->name,
                $sensorDTO->petId
            ));
            $this->createMeasurements($sensorDTO);
        };
    }

    private function createMeasurements(SensorDTO $sensorDTO): void
    {
        $now = time();
        $futures = [];
        for ($ts = $now - self::MEASUREMENT_PERIOD; $ts <= $now; $ts += self::MEASUREMENT_INTERVAL) {
            $futures[] = $this->sensorRepository->createMeasurement(
                $sensorDTO->id,
                new Timestamp($ts, 0),
                $this->readSensorData($sensorDTO->type->name)
            );
        }

        foreach ($futures as $future) {
            $future->get(Connector::BASE_TIMEOUT);
        }
    }

    private function readSensorData(string $type): float
    {
        return match ($type) {
            // Temperature in Fahrenheit
            'T' => 101 + random_int(0, 9) - 4,
            // Pulse in beats per minute
            'P' => 100 + random_int(0, 39) - 20,
            // Respiration in inhales per minute
            'R' => 35 + random_int(0, 4) - 2,
            // Location as distance from home
            'L' => round(10 * mt_rand() / mt_getrandmax(), 2),
        };
    }
}
