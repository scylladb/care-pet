<?php

namespace App\Core\Commands;

use App\Core\Commands\Base\AbstractCommand;
use App\Owner\OwnerFactory;
use App\Owner\OwnerRepository;
use App\Pet\PetFactory;
use App\Pet\PetRepository;
use App\Sensors\Sensor\SensorDTO;
use App\Sensors\Sensor\SensorFactory;
use App\Sensors\Sensor\SensorRepository;
use Closure;

final class SimulateCommand extends AbstractCommand
{

    /** @var OwnerRepository */
    private $ownerRepository;

    /** @var \App\Pet\PetRepository */
    private $petRepository;

    /** @var \App\Sensors\Sensor\SensorRepository */
    private $sensorRepository;

    public function __construct(
        OwnerRepository  $ownerRepository,
        PetRepository    $petRepository,
        SensorRepository $sensorRepository
    )
    {
        $this->ownerRepository = $ownerRepository;
        $this->petRepository = $petRepository;
        $this->sensorRepository = $sensorRepository;
    }

    const AMOUNT_BASE = 50000;

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
        };
    }
}
