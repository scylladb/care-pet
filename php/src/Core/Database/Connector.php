<?php

namespace App\Core\Database;

use Cassandra;
use Cassandra\Cluster;
use Cassandra\Cluster\Builder;
use Cassandra\FutureRows;
use Cassandra\Session;
use Cassandra\SimpleStatement;

class Connector
{
    public Builder $connectionBuilder;
    public Cluster $cluster;
    public Session $session;
    public SimpleStatement $query;

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

        $this->session = $this->cluster->connect($config['keyspace']);
    }

    public function setKeyspace(string $keyspace = ''): self
    {
        $this->session->close(self::BASE_TIMEOUT);
        $this->session = $this->cluster->connect($keyspace);

        return $this;
    }

    public function prepare(string $query): self
    {
        $this->query = new SimpleStatement($query);

        return $this;
    }

    public function execute(): FutureRows
    {
        return $this->session->executeAsync($this->query, []);
    }
}
