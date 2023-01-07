<?php

namespace App\Core\Entities;

abstract class AbstractFactory
{
    public abstract static function make(array $fields = []);

    public abstract static function makeMany(int $amount, array $fields = []);

}