using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using Cassandra; // DataStax Cassandra C# driver
using Microsoft.Extensions.Logging;
using CarePet.Model;
using System.CommandLine;
using System.CommandLine.Invocation;

namespace CarePet
{
    public class Sensor
    {
        private static readonly ILogger<Sensor> LOG;
        private readonly SensorConfig _config;
        private readonly Model.Owner _owner;
        private readonly Model.Pet _pet;
        private readonly Model.Sensor[] _sensors;

        static Sensor()
        {
            using var loggerFactory = LoggerFactory.Create(builder =>
            {
                builder.AddConsole();
            });
            LOG = loggerFactory.CreateLogger<Sensor>();
        }

        public Sensor(SensorConfig config)
        {
            _config = config;
            _owner = Owner.Random();
            _pet = Pet.Random(_owner.OwnerId);
            _sensors = new CarePet.Model.Sensor[Enum.GetValues(typeof(SensorType)).Length];

            var sensorTypes = Enum.GetValues(typeof(SensorType)).Cast<SensorType>().ToArray();
            for (int i = 0; i < _sensors.Length; i++)
            {
                _sensors[i] = new CarePet.Model.Sensor(_pet.PetId, Guid.NewGuid(), SensorTypeExtensions.GetTypeCode(sensorTypes[i]));
            }
        }

        public static void Main(string[] args)
        {
            var config = SensorConfig.Parse(args);
            var client = new Sensor(config);
            client.Save();
            client.Run();
        }

        /// <summary>
        /// Initiates a connection with the configured keyspace.
        /// </summary>
        public ISession Keyspace()
        {
            var session = _config.Builder(Config.Keyspace).Build().Connect();
            session.ChangeKeyspace(Config.Keyspace);
            return session;
        }

        /// <summary>
        /// Save owner, pet, and sensors to the database.
        /// </summary>
        private void Save()
        {
            using (var session = Keyspace())
            {
                var mapper = new Mapper(session);
                LOG.LogInformation($"owner = {_owner}");
                LOG.LogInformation($"pet = {_pet}");

                mapper.Owner().Create(_owner);
                mapper.Pet().Create(_pet);

                foreach (var s in _sensors)
                {
                    LOG.LogInformation($"sensor = {s}");
                    mapper.Sensor().Create(s);
                }
            }
        }

        /// <summary>
        /// Generate random sensor data and push it to the database.
        /// </summary>
        private void Run()
        {
            using (var session = Keyspace())
            {
                var prepared = session.Prepare("INSERT INTO measurement (sensor_id, ts, value) VALUES (?, ?, ?)");
                var ms = new List<Measure>();
                var prev = DateTimeOffset.UtcNow;

                while (true)
                {
                    while ((DateTimeOffset.UtcNow - prev) < _config.BufferInterval)
                    {
                        if (!Sleep(_config.Measurement))
                            return;

                        foreach (var s in _sensors)
                        {
                            var m = ReadSensorData(s);
                            ms.Add(m);
                            LOG.LogInformation(m.ToString());
                        }
                    }

                    var elapsed = DateTimeOffset.UtcNow - prev;
                    var intervals = elapsed.Ticks / _config.BufferInterval.Ticks;
                    prev = prev.AddTicks(intervals * _config.BufferInterval.Ticks);

                    LOG.LogInformation("pushing data");

                    var batch = new BatchStatement();
                    foreach (var m in ms)
                    {
                        batch.Add(prepared.Bind(m.SensorId, m.Ts.UtcDateTime, m.Value));
                    }

                    session.Execute(batch);
                    ms.Clear();
                }
            }
        }

        private bool Sleep(TimeSpan interval)
        {
            try
            {
                Thread.Sleep(interval);
                return true;
            }
            catch (ThreadInterruptedException)
            {
                return false;
            }
        }

        private Measure ReadSensorData(CarePet.Model.Sensor s)
        {
            return new Measure(s.SensorId, DateTimeOffset.UtcNow, s.RandomData());
        }

        public class SensorConfig : Config
        {
            public TimeSpan BufferInterval { get; set; } = TimeSpan.FromHours(1);
            public TimeSpan Measurement { get; set; } = TimeSpan.FromMinutes(1);

            public static SensorConfig Parse(string[] args)
            {
                var config = new SensorConfig();

                // Base options from Config
                var hostsOption = new Option<string[]>("--hosts", "Database contact points");
                var dcOption = new Option<string>(new[] { "-dc", "--datacenter" }, "Local datacenter name");
                var usernameOption = new Option<string>(new[] { "-u", "--username" }, "Authentication username");
                var passwordOption = new Option<string>(new[] { "-p", "--password" }, "Authentication password");
                var helpOption = new Option<bool>(new[] { "-h", "--help" }, "Display help message");

                var bufferInterval = new Option<TimeSpan>(
                        "--buffer-interval",
                        "Buffer interval to accumulate measures");
                var measure = new Option<TimeSpan>(
                        "--measure",
                        "Sensors measurement interval");
                var rootCommand = new RootCommand
                {
                    hostsOption,
                    dcOption,
                    usernameOption,
                    passwordOption,
                    helpOption,
                    bufferInterval,
                    measure
                };

                rootCommand.SetHandler((InvocationContext context) =>
                {
                    config.Hosts = context.ParseResult.GetValueForOption(hostsOption);
                    config.Datacenter = context.ParseResult.GetValueForOption(dcOption);
                    config.Username = context.ParseResult.GetValueForOption(usernameOption);
                    config.Password = context.ParseResult.GetValueForOption(passwordOption);
                    config.Help = context.ParseResult.GetValueForOption(helpOption);

                    config.BufferInterval = context.ParseResult.GetValueForOption(bufferInterval);
                    config.Measurement = context.ParseResult.GetValueForOption(measure);
                });

                rootCommand.InvokeAsync(args);

                if (config.Help == true)
                {
                    rootCommand.InvokeAsync("-h");
                    Environment.Exit(1);
                }

                return config;
            }
        }
    }
}
