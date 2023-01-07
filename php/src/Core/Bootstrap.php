<?php
namespace App\Core;

use App\Core\Database\Connector;
use DI\Container;
use DI\ContainerBuilder;
use Dotenv\Dotenv;
use Psr\Container\ContainerInterface;

class Bootstrap
{
    public function init(): Container
    {
        // Loading the environment variables
        $dotenv = Dotenv::createImmutable(basePath());
        $dotenv->load();

        // Loading the base container
        $container = new ContainerBuilder();
        $container->addDefinitions([
            Connector::class => function (ContainerInterface $c) {
                return new Connector(config('database'));
            },
        ]);

        return $container->build();
    }
}