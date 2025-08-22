using System;
using Cassandra.Mapping.Attributes;
using Newtonsoft.Json;

namespace CarePet.Model
{
    [Table(Name = "measurement")]
    public class Measure
    {
        [PartitionKey]
        [Column("sensor_id")]
        [JsonProperty("sensor_id")]
        public Guid SensorId { get; set; }

        [ClusteringKey]
        [Column("ts")]
        public DateTimeOffset Ts { get; set; }

        [Column("value")]
        public float Value { get; set; }

        public Measure() { }

        public Measure(Guid sensorId, DateTimeOffset ts, float value)
        {
            SensorId = sensorId;
            Ts = ts;
            Value = value;
        }

        public override string ToString()
        {
            return $"Measure{{sensorId={SensorId}, ts={Ts}, value={Value}}}";
        }
    }
}
