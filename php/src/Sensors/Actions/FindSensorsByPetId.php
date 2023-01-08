<?php

namespace App\Sensors\Actions;

use App\Pet\Actions\FindPetById;
use App\Sensors\Sensor\SensorCollection;
use App\Sensors\Sensor\SensorDTO;
use App\Sensors\Sensor\SensorRepository;
use App\Sensors\Sensor\SensorsException;

class FindSensorsByPetId
{
    /** @var SensorRepository */
    private $sensorRepository;

    /** @var FindPetById */
    private $findPetAction;

    public function __construct(SensorRepository $sensorRepository, FindPetById $findPetAction)
    {
        $this->sensorRepository = $sensorRepository;
        $this->findPetAction = $findPetAction;
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