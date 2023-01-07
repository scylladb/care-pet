<?php

namespace App;

use App\Owner\Controllers\FindOwnerController;
use App\Owner\Controllers\OwnerPetsController;
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
            $router->get('/pets/{id:\d+}/', [x::class, 'handle']);
            $router->get('/pets/{id:\d+}/sensors', [x::class, 'handle']);
            $router->get('/sensors/{id:\d+}', [x::class, 'handle']);
            $router->get('/sensors/{id:\d+}/daily', [x::class, 'handle']);
        });
    }
}