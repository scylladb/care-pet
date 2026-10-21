<?php

namespace App\Core;


use App\Router;
use FastRoute\Dispatcher;
use Throwable;

class Kernel
{
    public function run(): void
    {
        $container = (new Bootstrap())->init();
        $uri = $this->resolveRequestUri();
        $httpMethod = $this->resolveHttpMethod();

        $routeInfo = Router::map()->dispatch($httpMethod, $uri);
        switch ($routeInfo[0]) {
            case Dispatcher::NOT_FOUND:
                http_response_code(404);
                break;
            case Dispatcher::METHOD_NOT_ALLOWED:
                http_response_code(405);
                break;
            case Dispatcher::FOUND:
                [$controller] = $routeInfo[1];
                $params = $routeInfo[2];
                try {
                    $container->call($controller, $params);
                } catch (Throwable $e) {
                    $this->handleException($e);
                }
                break;
        }

    }



    // Domain exceptions use the HTTP status code as the exception code.
    private function handleException(Throwable $e): void
    {
        $status = $e->getCode() >= 400 && $e->getCode() < 500 ? $e->getCode() : 500;
        $message = $status === 500 ? 'Internal server error.' : $e->getMessage();
        if ($status === 500) {
            error_log((string)$e);
        }

        header('Content-type: application/json');
        http_response_code($status);
        echo json_encode(['error' => $message]);
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
