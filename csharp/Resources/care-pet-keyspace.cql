CREATE KEYSPACE IF NOT EXISTS carepet
WITH replication = {
    'class': 'NetworkTopologyStrategy',
    'datacenter1': 3
} AND durable_writes = true;
