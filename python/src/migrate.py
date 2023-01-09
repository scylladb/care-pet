import os
from db import config
from db.client import ScyllaClient

arg_parser = config.argument_parser()
db_config = vars(arg_parser.parse_args())
client = ScyllaClient(db_config)
session = client.get_session()


def absolute_file_path(relative_file_path):
    current_dir = os.path.dirname(__file__)
    return os.path.join(current_dir, relative_file_path)


print("Creating keyspace...")
with open(absolute_file_path("db/cql/keyspace.cql"), "r") as file:
    session.execute(file.read())
print("Done.")

print("Migrating database...")
with open(absolute_file_path("db/cql/migrate.cql"), "r") as file:
    for query in file.read().split(";"):
        if len(query) > 0:
            session.execute(query)
print("Done.")

client.shutdown()
