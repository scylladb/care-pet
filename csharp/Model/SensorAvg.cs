using System;
using Cassandra.Mapping.Attributes;
using Newtonsoft.Json;

namespace CarePet.Model
{
    [Table(Name = "sensor_avg")]
    public class SensorAvg
    {
        [PartitionKey]
        [Column("sensor_id")]
        [JsonProperty("sensor_id")]
        public Guid SensorId { get; set; }

        [ClusteringKey(0)]
        [Column("date")]
        public DateTime Date { get; set; }

        [ClusteringKey(1)]
        [Column("hour")]
        public int Hour { get; set; }

        [Column("value")]
        public float Value { get; set; }

        public SensorAvg() { }

        public SensorAvg(Guid sensorId, DateTime date, int hour, float value)
        {
            SensorId = sensorId;
            Date = date;
            Hour = hour;
            Value = value;
        }

        public override string ToString()
        {
            return $"SensorAvg{{sensorId={SensorId}, date={Date}, hour={Hour}, value={Value}}}";
        }
    }
}
