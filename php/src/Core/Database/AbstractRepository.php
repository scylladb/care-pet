<?php

namespace App\Core\Database;

use App\Core\Entities\AbstractDTO;
use Cassandra\Rows;

abstract class AbstractRepository
{
    public string $table = '';

    public string $primaryKey = '';

    public Connector $connection;

    public array $keys = [];

    public function __construct(Connector $connector)
    {
        $this->connection = $connector;
    }

    public function getById(string $id): Rows
    {
        $query = sprintf("SELECT * FROM %s WHERE %s = %s", $this->table, $this->primaryKey, $id);

        return $this->connection
            ->prepare($query)
            ->execute()
            ->get(Connector::BASE_TIMEOUT);
    }

    public function all(): Rows
    {
        return $this->connection
            ->prepare(sprintf('SELECT * FROM %s', $this->table))
            ->execute()
            ->get(Connector::BASE_TIMEOUT);
    }

    public function create(AbstractDTO $dto): void
    {
        $keys = array_keys($dto->toDatabase());
        $dataValues = array_values($dto->toDatabase());

        foreach ($dataValues as $key => $value) {
            if (is_string($value) && !in_array($keys[$key], $this->keys)) {
                $value = addslashes($value);
                $dataValues[$key] = "'$value'";
            }
        }

        $query = sprintf(
            "INSERT INTO %s (%s) VALUES (%s)",
            $this->table,
            implode(', ', $keys),
            implode(', ', $dataValues)
        );


        $this->connection
            ->prepare($query)
            ->execute();
    }
}
