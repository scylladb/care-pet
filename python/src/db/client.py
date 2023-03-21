
#!/usr/bin/python
#
# A simple example of connecting to a cluster
# To install the driver Run pip install scylla-driver
from cassandra.cluster import Cluster, ExecutionProfile, EXEC_PROFILE_DEFAULT
from cassandra.policies import DCAwareRoundRobinPolicy, TokenAwarePolicy
from cassandra.auth import PlainTextAuthProvider
from cassandra.query import BatchStatement, dict_factory
from cassandra import ConsistencyLevel
import uuid


class ScyllaClient():
    
    def __init__(self, config):
        self.cluster = self._get_cluster(config)
        self.session = self.cluster.connect()
        
    def __enter__(self):
        return self
    
    def __exit__(self, exc_type, exc_value, traceback):
        self.shutdown()
        
    def shutdown(self):
        self.cluster.shutdown()

    def _get_cluster(self, config):
        profile = ExecutionProfile(
            load_balancing_policy=TokenAwarePolicy(
                    DCAwareRoundRobinPolicy(local_dc=config["datacenter"])
                ),
                row_factory=dict_factory
            )
        return Cluster(
            execution_profiles={EXEC_PROFILE_DEFAULT: profile},
            contact_points=[config["hosts"]],
            auth_provider = PlainTextAuthProvider(username=config["username"],
                                                  password=config["password"]))
    
    def print_metadata(self):
        for host in self.cluster.metadata.all_hosts():
            print(f"Datacenter: {host.datacenter}; Host: {host.address}; Rack: {host.rack}")
    
    def get_session(self):
        return self.session
    
    def insert_data(self, table, data_dict):
        columns = list(data_dict.keys())
        values = list(data_dict.values())
        insert_query = f"""
        INSERT INTO {table} ({','.join(columns)}) 
        VALUES ({','.join(['%s' for c in columns])});
        """
        self.session.execute(insert_query, values)
        
    def insert_batch_data(self, table, data_list):
        columns = list(data_list[0].keys())
        insert_batch_query = self.session.prepare(f"""
            INSERT INTO {table} ({','.join(columns)}) VALUES ({','.join(['?' for c in columns])})
            """)
        batch = BatchStatement(consistency_level=ConsistencyLevel.QUORUM)
        for data_dict in data_list:
            values = list(data_dict.values())
            batch.add(insert_batch_query, values)
        self.session.execute(batch)

    def fetch_owner(self, owner_id):
        query = "SELECT * FROM carepet.owner WHERE owner_id = %s;"
        return self.session.execute(query, (uuid.UUID(owner_id), ))
    
    def fetch_pets(self, owner_id):
        query = "SELECT * FROM carepet.pet WHERE owner_id = %s;"
        return self.session.execute(query, (uuid.UUID(owner_id), ))   
    
    def fetch_sensors(self, pet_id):
        query = "SELECT * FROM carepet.sensor WHERE pet_id = %s;"
        return self.session.execute(query, (uuid.UUID(pet_id), ))
        
    