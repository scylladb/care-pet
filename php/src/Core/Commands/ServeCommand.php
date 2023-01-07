<?php

namespace App\Core\Commands;

use App\Core\Commands\Base\AbstractCommand;

class ServeCommand extends AbstractCommand
{
    public function __invoke(array $args = []): int
    {
        $this->info('CarePet Web started!');
        $this->info('Development Server: http://0.0.0.0:8000');
        echo `php -S 0.0.0.0:8000 -t public`;

        return self::SUCCESS;
    }
}