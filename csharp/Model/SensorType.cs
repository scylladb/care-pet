using System;

namespace CarePet.Model
{
    public enum SensorType
    {
        Temperature,
        Pulse,
        Location,
        Respiration
    }

    public static class SensorTypeExtensions
    {
        public static string GetTypeCode(this SensorType sensorType)
        {
            return sensorType switch
            {
                SensorType.Temperature => "T",
                SensorType.Pulse => "P",
                SensorType.Location => "L",
                SensorType.Respiration => "R",
                _ => throw new ArgumentOutOfRangeException(nameof(sensorType), sensorType, null)
            };
        }

        public static SensorType FromString(string text)
        {
            return text?.ToUpperInvariant() switch
            {
                "T" => SensorType.Temperature,
                "P" => SensorType.Pulse,
                "L" => SensorType.Location,
                "R" => SensorType.Respiration,
                _ => throw new ArgumentException("Invalid sensor type code", nameof(text))
            };
        }
    }
}
