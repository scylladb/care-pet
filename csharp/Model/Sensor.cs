using System;
using Cassandra.Mapping.Attributes;
using Newtonsoft.Json;

namespace CarePet.Model
{
    [Table(Name = "sensor")]
    public class Sensor
    {
        [PartitionKey]
        [JsonProperty("pet_id")]
        [Column("pet_id")]
        public Guid PetId { get; set; }

        [ClusteringKey]
        [JsonProperty("sensor_id")]
        [Column("sensor_id")]
        public Guid SensorId { get; set; }

        public string Type { get; set; }

        public Sensor()
        {
        }

        public Sensor(Guid petId, Guid sensorId, string type)
        {
            PetId = petId;
            SensorId = sensorId;
            Type = type;
        }

        public static Sensor Random(Guid petId)
        {
            var types = Enum.GetValues(typeof(SensorType));
            var random = new Random();
            var randomType = (SensorType)types.GetValue(random.Next(types.Length));
            var typeCode = SensorTypeExtensions.GetTypeCode(randomType);
            return new Sensor(petId, Guid.NewGuid(), typeCode);
        }

        public float RandomData()
        {
            var random = new Random();
            switch (SensorTypeExtensions.FromString(Type))
            {
                case SensorType.Temperature:
                    // average F
                    return 101.0f + random.Next(10) - 4;
                case SensorType.Pulse:
                    // average beats per minute
                    return 100.0f + random.Next(40) - 20;
                case SensorType.Respiration:
                    // average inhales per minute
                    return 35.0f + random.Next(5) - 2;
                case SensorType.Location:
                    // pet can teleport
                    return (float)(10 * random.NextDouble());
                default:
                    return 0.0f;
            }
        }

        public override string ToString()
        {
            return $"Sensor{{petId={PetId}, sensorId={SensorId}, type='{Type}'}}";
        }
    }
}
