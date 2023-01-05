<?php

namespace App\Core\Commands;

use App\Core\Commands\Base\AbstractCommand;
use App\Owner\OwnerDTO;
use App\Owner\OwnerFactory;
use App\Owner\OwnerRepository;
use App\Pet\PetFactory;
use App\Pet\PetRepository;
use App\Sensor\SensorFactory;
use App\Sensor\SensorRepository;

class SeedCommand extends AbstractCommand
{

    const AMOUNT_BASE = 50000;

    public function handle(array $args): int
    {
        $ownerRepository = new OwnerRepository();
        $petRepository = new PetRepository();
        $sensorRepository = new SensorRepository();

        foreach (range(0, self::AMOUNT_BASE) as $i) {
            $this->info("Batch: " . $i);
            $ownerDTO = OwnerFactory::make();
            $petDTO = PetFactory::make(['owner_id' => $ownerDTO->id]);
            $sensorDTOs = SensorFactory::makeMany(5, [
                'pet_id' => $petDTO->id,
                'owner_id' => $petDTO->id
            ]);

            $ownerRepository->create($ownerDTO);
            $this->info(sprintf('Owner %s', $ownerDTO->id));

            $petRepository->create($petDTO);
            $this->info(sprintf('Pet: %s | Owner %s', $petDTO->id, $petDTO->ownerId));

            foreach ($sensorDTOs as $sensorDTO) {
                $sensorRepository->create($sensorDTO);
                $this->info(sprintf('Sensor: %s | Pet %s', $sensorDTO->id, $sensorDTO->petId));
            }
        }
        $this->info('Done :D');

        return self::SUCCESS;
    }
}