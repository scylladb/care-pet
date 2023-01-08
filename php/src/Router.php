<?php

namespace App;

use App\Owner\Controllers\FindOwnerController;
use App\Owner\Controllers\OwnerPetsController;
use App\Pet\Controllers\FindPetController;
use App\Sensors\Controllers\GetPetSensorsController;
use App\Sensors\Controllers\SensorsByDateController;
use FastRoute\Dispatcher;
use FastRoute\RouteCollector;
use function FastRoute\simpleDispatcher;

class Router
{
    public static function map(): Dispatcher
    {
        return simpleDispatcher(function(RouteCollector $router) {
            $router->get('/owners/{ownerId}', [FindOwnerController::class]);
            $router->get('/owners/{ownerId}/pets', [OwnerPetsController::class]);
            $router->get('/pets/{petId}', [FindPetController::class]);
            $router->get('/pets/{petId}/sensors', [GetPetSensorsController::class]);
            $router->get('/sensors/{sensorId}/values', [SensorsByDateController::class]);
            $router->get('/sensors/{id:\d+}/daily', [x::class, 'handle']);
        });
    }
}
