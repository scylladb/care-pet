<?php

namespace App\Core\Entities;

abstract class AbstractFactory
{
    /** @return AbstractDTO */
    public abstract static function make(array $fields = []);

    /** @return Collection<int, AbstractDTO> */
    public abstract static function makeMany(int $amount, array $fields = []);

}
