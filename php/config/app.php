<?php


return [
    'database' => [
        'nodes' => env('DB_NODES', 'carepet-scylla1'),
        'keyspace' => env('DB_KEYSPACE', 'carepet'),
        'username' => env('DB_USERNAME'),
        'password' => env('DB_PASSWORD'),
        'consistency_level' => Cassandra::CONSISTENCY_QUORUM,
        'port' => (int) env('DB_PORT', 9042)
    ]
];