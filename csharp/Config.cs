using System;
using System.IO;
using System.Linq;
using System.Net;
using System.Text;
using Cassandra;
using System.Reflection;
using System.CommandLine;
using System.CommandLine.Invocation;

namespace CarePet
{
    public class Config
    {
        public const string Keyspace = "carepet";
        private const string ApplicationName = "care-pet";
        private static readonly Guid ClientId = Guid.NewGuid();
        private const int DefaultPort = 9042;

        // Command-line options
        public string[] Hosts { get; set; }
        public string Datacenter { get; set; }
        public string Username { get; set; }
        public string Password { get; set; }
        public bool Help { get; set; }

        /// <summary>
        /// Parses arguments into a new instance of Config.
        /// </summary>
        public static T Parse<T>(T command, string[] args) where T : class
        {
            var hostsOption = new Option<string[]>(
                "--hosts",
                "Database contact points");
            var dcOption = new Option<string>(
                new[] { "-dc", "--datacenter" },
                "Local datacenter name for default profile");
            var usernameOption = new Option<string>(
                new[] { "-u", "--username" },
                "Password based authentication username");
            var passwordOption = new Option<string>(
                new[] { "-p", "--password" },
                "Password based authentication password");
            var helpOption = new Option<bool>(
                new[] { "-h", "--help" },
                "Display a help message");
            var rootCommand = new RootCommand
            {
                hostsOption,
                dcOption,
                usernameOption,
                passwordOption,
                helpOption
            };

            rootCommand.SetHandler((InvocationContext context) =>
            {
                if (command is Config cfg)
                {
                    cfg.Hosts = context.ParseResult.GetValueForOption(hostsOption);
                    cfg.Datacenter = context.ParseResult.GetValueForOption(dcOption);
                    cfg.Username = context.ParseResult.GetValueForOption(usernameOption);
                    cfg.Password = context.ParseResult.GetValueForOption(passwordOption);
                    cfg.Help = context.ParseResult.GetValueForOption(helpOption);
                }
            });

            rootCommand.InvokeAsync(args).Wait();

            if ((command as Config)?.Help == true)
            {
                rootCommand.InvokeAsync("-h");
                Environment.Exit(1);
            }

            return command;
        }

        /// <summary>
        /// Transforms an address of the form host:port into an IPEndPoint.
        /// </summary>
        public static IPEndPoint Resolve(string addr)
        {
            var addressWithPort = WithPort(addr, DefaultPort);
            var parts = addressWithPort.Split(':');

            if (parts.Length != 2 || !int.TryParse(parts[1], out int port))
            {
                throw new UriFormatException("URI must have host and port");
            }

            return new IPEndPoint(Dns.GetHostAddresses(parts[0]).First(), port);
        }

        /// <summary>
        /// Ensures an address has port provided.
        /// </summary>
        private static string WithPort(string addr, int port)
        {
            if (!addr.Contains(":"))
            {
                return $"{addr}:{port}";
            }
            return addr;
        }

        /// <summary>
        /// Check if string is null or empty.
        /// </summary>
        public static bool IsNullOrEmpty(string s)
        {
            return string.IsNullOrEmpty(s);
        }

        /// <summary>
        /// Loads a resource content.
        /// </summary>
        public static string GetResource(string name)
        {
            return GetResourceFileAsString(name);
        }

        /// <summary>
        /// Reads given resource file as a string.
        /// </summary>
        private static string GetResourceFileAsString(string name)
        {
            var assembly = Assembly.GetExecutingAssembly();

            var fullName = assembly.GetName().Name + ".Resources." + name;

            using var stream = assembly.GetManifestResourceStream(fullName)
                ?? throw new FileNotFoundException($"Resource '{fullName}' not found");

            using var reader = new StreamReader(stream);
            return reader.ReadToEnd();
        }

        /// <summary>
        /// Builds configured Cassandra Cluster builder to acquire a new session.
        /// </summary>
        public Cassandra.Builder Builder(string keyspace = null)
        {
            var builder = Cassandra.Cluster.Builder();

            if (Hosts != null && Hosts.Length > 0)
            {
                builder = builder.AddContactPoints(Hosts)
                                 .WithLoadBalancingPolicy(new DCAwareRoundRobinPolicy(Datacenter));
            }

            if (!IsNullOrEmpty(Username))
            {
                builder = builder.WithCredentials(Username, Password);
            }

            return builder;
        }
    }
}
