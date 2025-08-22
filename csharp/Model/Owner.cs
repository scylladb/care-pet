using System;
using Cassandra.Mapping.Attributes;
using Newtonsoft.Json;

namespace CarePet.Model
{
    [Table(Name = "owner")]
    public class Owner
    {
        [PartitionKey]
        [Column("owner_id")]
        [JsonProperty("owner_id")]
        public Guid OwnerId { get; set; }

        [Column("name")]
        public string Name { get; set; }

        [Column("address")]
        public string Address { get; set; }

        public Owner(Guid ownerId, string name, string address)
        {
            OwnerId = ownerId;
            Name = name;
            Address = address;
        }

        public Owner()
        {
        }

        public static Owner Random()
        {
            return new Owner(
                Guid.NewGuid(),
                RandomString(8),
                RandomString(10)
            );
        }

        private static string RandomString(int length)
        {
            const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            var random = new Random();
            var result = new char[length];
            for (int i = 0; i < length; i++)
            {
                result[i] = chars[random.Next(chars.Length)];
            }
            return new string(result);
        }

        public override string ToString()
        {
            return $"Owner{{ownerId={OwnerId}, name='{Name}', address='{Address}'}}";
        }
    }
}
