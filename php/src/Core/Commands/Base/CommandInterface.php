<?php
namespace App\Core\Commands\Base;

interface CommandInterface
{
    public function handle(array $args): int;
}