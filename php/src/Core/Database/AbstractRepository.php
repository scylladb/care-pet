<?php

namespace App\Core\Database;

use App\Core\Entities\AbstractDTO;
use Cassandra\Rows;
use Cassandra\Uuid;

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

    public function getById(Uuid $id): Rows
    {
        $query = sprintf('SELECT * FROM %s WHERE %s = ?', $this->table, $this->primaryKey);

        return $this->connection
            ->prepare($query)
            ->execute([$id])
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
        $values = $dto->toDatabase();

        // Prepared statements need driver types for UUID and FLOAT columns.
        foreach ($values as $column => $value) {
            if (in_array($column, $this->keys)) {
                $values[$column] = new Uuid($value);
            } elseif (is_float($value)) {
                $values[$column] = new \Cassandra\Float($value);
            }
        }

        $query = sprintf(
            'INSERT INTO %s (%s) VALUES (%s)',
            $this->table,
            implode(', ', array_keys($values)),
            implode(', ', array_fill(0, count($values), '?'))
        );

        $this->connection
            ->prepare($query)
            ->execute(array_values($values))
            ->get(Connector::BASE_TIMEOUT);
    }
}
