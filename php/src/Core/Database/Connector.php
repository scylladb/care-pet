<?php

namespace App\Core\Database;

use Cassandra;
use Cassandra\Cluster;
use Cassandra\Cluster\Builder;
use Cassandra\FutureRows;
use Cassandra\PreparedStatement;
use Cassandra\Session;

class Connector
{
    public Builder $connectionBuilder;
    public Cluster $cluster;
    public Session $session;
    public PreparedStatement $query;

    /** @var array<string, PreparedStatement> */
    private array $preparedStatements = [];

    const BASE_TIMEOUT = 10;

    public function __construct(array $config)
    {
        $this->connectionBuilder = Cassandra::cluster()
            ->withContactPoints($config['nodes'])
            ->withDefaultConsistency($config['consistency_level'])
            ->withPort($config['port']);

        if (!empty($config['username'] && !empty($config['password']))) {
            $this->connectionBuilder = $this->connectionBuilder->withCredentials($config['username'], $config['password']);
        }
        $this->cluster = $this->connectionBuilder->build();

        // The keyspace does not exist before the first migration.
        $this->session = $this->cluster->connect();
        if ($this->session->schema()->keyspace($config['keyspace']) !== false) {
            $this->setKeyspace($config['keyspace']);
        }
    }

    public function setKeyspace(string $keyspace = ''): self
    {
        $this->session->close(self::BASE_TIMEOUT);
        $this->session = $this->cluster->connect($keyspace);
        $this->preparedStatements = [];

        return $this;
    }

    // Prepare each query once. Send the input as bound values in execute().
    public function prepare(string $query): self
    {
        $this->query = $this->preparedStatements[$query] ??= $this->session->prepare($query);

        return $this;
    }

    public function execute(array $arguments = []): FutureRows
    {
        return $this->session->executeAsync($this->query, ['arguments' => $arguments]);
    }
}
