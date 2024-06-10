<?php

namespace App\Core\Database;

use App\Core\Entities\AbstractDTO;
use Cassandra\PreparedStatement;
use Cassandra\Rows;
use Cassandra\SimpleStatement;

abstract class AbstractRepository
{
    public string $table = '';

    public string $primaryKey = '';

    public Connector $connection;

    /**
     * @var array<string, PreparedStatement>
     */
    protected array $preparedStatements = [];

    public array $keys = [];

    public function __construct(Connector $connector)
    {
        $this->connection = $connector;

        $this->preparedStatements = [
            'create' => $this->connection->session->prepare(sprintf(
                "INSERT INTO %s (%s) VALUES (%s)",
                $this->table,
                implode(', ', $this->keys),
                implode(', ', array_fill(0, count($this->keys), '?'))
            ))
        ];
    }

    public function getById(string $id): Rows
    {
        $query = sprintf("SELECT * FROM %s WHERE %s = ?", $this->table, $this->primaryKey);

        $prepared = $this->connection
            ->session
            ->prepare($query);

        return $this->connection
            ->session
            ->execute($prepared, [$id]);
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

        $this->connection
            ->session
            ->executeAsync($this->preparedStatements['create'], ['arguments' => $dto->toDatabase()]);

    }
}
