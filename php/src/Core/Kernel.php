<?php

namespace App\Core;


use App\Router;
use FastRoute\Dispatcher;

class Kernel
{
    public function run()
    {
        $container = (new Bootstrap())->init();
        $uri = $this->resolveRequestUri();
        $httpMethod = $this->resolveHttpMethod();

        $routeInfo = Router::map()->dispatch($httpMethod, $uri);
        switch ($routeInfo[0]) {
            case Dispatcher::NOT_FOUND:
                echo 'rota errada irmão';
                break;
            case Dispatcher::METHOD_NOT_ALLOWED:
                $allowedMethods = $routeInfo[1];
                // ... 405 Method Not Allowed
                break;
            case Dispatcher::FOUND:
                [$controller] = $routeInfo[1];
                $params = $routeInfo[2];
                $container->call($controller,$params);
                break;
        }

    }



    public function resolveHttpMethod(): string
    {
        return $_SERVER['REQUEST_METHOD'];
    }

    public function resolveRequestUri(): string
    {
        $uri = $_SERVER['REQUEST_URI'];
        if (false !== $pos = strpos($uri, '?')) {
            $uri = substr($uri, 0, $pos);
        }

        return rawurldecode($uri);
    }
}