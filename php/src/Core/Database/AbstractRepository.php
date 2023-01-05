<?php

namespace App\Core\Database;

use App\Core\AbstractDTO;
use App\Owner\OwnerDTO;
use Cassandra\FutureRows;
use Cassandra\Rows;

abstract class AbstractRepository
{

    public $table = '';

    public $primaryKey = '';

    /**
     * @var Connector
     */
    public $connection;

    public function __construct()
    {
        $this->connection = new Connector(config('database'));
    }

    public function getById(string $id): Rows
    {
        return $this->connection
            ->prepare(sprintf('SELECT * FROM %s WHERE %s = %s', $this->table, $this->primaryKey, $id))
            ->execute()
            ->get(5);
    }

    public function all(): Rows
    {
        return $this->connection
            ->prepare(sprintf('SELECT * FROM %s', $this->table))
            ->execute()
            ->get(5);
    }

    public function create(AbstractDTO $dto): bool
    {
        $keys = array_keys($dto->toDatabase());
        $dataValues = array_values($dto->toDatabase());

        foreach ($dataValues as $key => $value) {
            if (is_string($value)) {
                $dataValues[$key] = "'$value'";
            }
        }

        $query = sprintf(
            "INSERT INTO %s (%s, %s) VALUES (uuid(), %s)",
            $this->table,
            $this->primaryKey,
            implode(', ', $keys),
            implode(', ', $dataValues)
        );


        return (bool) $this->connection
            ->prepare($query)
            ->execute();
    }
}