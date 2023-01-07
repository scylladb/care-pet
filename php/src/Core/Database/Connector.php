<?php

namespace App\Core\Database;

use Cassandra;
use Cassandra\FutureRows;
use Cassandra\Session;
use Cassandra\SimpleStatement;

class Connector
{
    /** @var \Cassandra\Cluster\Builder */
    public $cluster;

    public $session;

    /** @var SimpleStatement */
    public $query;

    public function __construct(array $config)
    {
        $this->cluster = Cassandra::cluster()
            ->withContactPoints($config['nodes'])
            ->withDefaultConsistency($config['consistency_level'])
            ->withPort($config['port']);

        if (!empty($config['username'] && !empty($config['password']))) {
            $this->cluster = $this->cluster->withCredentials($config['username'], $config['password']);
        }
        $this->cluster = $this->cluster->build();
        $this->session = $this->cluster->connect($config['keyspace']);
    }

    public function setKeyspace(string $keyspace = ''): self
    {
        $this->session->close(10);
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