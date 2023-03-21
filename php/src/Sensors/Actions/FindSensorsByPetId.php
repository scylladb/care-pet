<?php

namespace App\Sensors\Actions;

use App\Pet\Actions\FindPetById;
use App\Sensors\Sensor\SensorCollection;
use App\Sensors\Sensor\SensorDTO;
use App\Sensors\Sensor\SensorRepository;
use App\Sensors\Sensor\SensorsException;

class FindSensorsByPetId
{

    public function __construct(
        private readonly SensorRepository $sensorRepository,
        private readonly FindPetById      $findPetAction
    )
    {
    }

    /** @return SensorCollection<int, SensorDTO> */
    public function handle(string $petId): SensorCollection
    {
        $petDTO = $this->findPetAction->handle($petId);
        $sensors = $this->sensorRepository->getSensorsByPetId($petDTO->id->uuid());

        if ($sensors->count() == 0) {
            throw SensorsException::noSensorsFound();
        }

        return SensorCollection::make($sensors);
    }
}