<?php

namespace App\Core\Commands\Base;

abstract class AbstractCommand implements CommandInterface
{
    const SUCCESS = 0;
    const FAIL = 1;

    public function info(string $message): void
    {
        echo sprintf('[INFO] %s %s', $message, PHP_EOL);
    }
}