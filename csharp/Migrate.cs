using System;
using System.Linq;
using Cassandra;
using Microsoft.Extensions.Logging;

namespace CarePet
{
    public class Migrate
    {
        private static readonly ILogger<Migrate> LOG;
        private readonly Config _config;

        static Migrate()
        {
            // You can wire up any logger provider here (Console, Serilog, etc.)
            using var loggerFactory = LoggerFactory.Create(builder =>
            {
                builder.AddConsole();
            });
            LOG = loggerFactory.CreateLogger<Migrate>();
        }

        public Migrate(Config config)
        {
            _config = config;
        }

        public static void Main(string[] args)
        {
            var config = Config.Parse(new Config(), args);

            var client = new Migrate(config);
            client.CreateKeyspace();
            client.CreateSchema();
            client.PrintMetadata();
        }

        /// <summary>
        /// Initiates a connection without a specific keyspace.
        /// </summary>
        public ISession Connect()
        {
            return _config.Builder().Build().Connect();
        }

        /// <summary>
        /// Initiates a connection with the configured keyspace.
        /// </summary>
        public ISession Keyspace()
        {
            return _config.Builder(Config.Keyspace).Build().Connect();
        }

        /// <summary>
        /// Creates the keyspace for this example.
        /// </summary>
        public void CreateKeyspace()
        {
            LOG.LogInformation("Creating keyspace carepet...");
            using (var session = Connect())
            {
                var cql = Config.GetResource("care-pet-keyspace.cql");
                if (!string.IsNullOrWhiteSpace(cql))
                {
                    session.Execute(cql);
                }
            }
            LOG.LogInformation("Keyspace carepet created successfully");
        }

        /// <summary>
        /// Creates the tables for this example.
        /// </summary>
        public void CreateSchema()
        {
            LOG.LogInformation("Creating tables...");
            using (var session = Keyspace())
            {
                var ddl = Config.GetResource("care-pet-ddl.cql");
                if (!string.IsNullOrWhiteSpace(ddl))
                {
                    var statements = ddl.Split(';')
                                        .Select(s => s.Trim())
                                        .Where(s => !string.IsNullOrEmpty(s));
                    foreach (var cql in statements)
                    {
                        session.Execute(cql);
                    }
                }
            }
        }

        /// <summary>
        /// Prints keyspace metadata.
        /// </summary>
        public void PrintMetadata()
        {
            using (var session = Keyspace())
            {
                var metadata = session.Cluster.Metadata;
                var ksMeta = metadata.GetKeyspace(Config.Keyspace);

                if (ksMeta != null)
                {
                    foreach (var table in ksMeta.GetTablesMetadata())
                    {
                        Console.WriteLine($"Keyspace: {Config.Keyspace}; Table: {table.Name}");
                    }
                }
            }
        }
    }
}
