using System;
using Cassandra.Mapping.Attributes;
using Newtonsoft.Json;

namespace CarePet.Model
{
    [Table(Name = "pet")]
    public class Pet
    {
        [PartitionKey]
        [Column("owner_id")]
        [JsonProperty("owner_id")]
        public Guid OwnerId { get; set; }

        [ClusteringKey]
        [Column("pet_id")]
        [JsonProperty("pet_id")]
        public Guid PetId { get; set; }

        [Column("chip_id")]
        [JsonProperty("chip_id")]
        public string ChipId { get; set; }

        [Column("species")]
        public string Species { get; set; }

        [Column("breed")]
        public string Breed { get; set; }

        [Column("color")]
        public string Color { get; set; }

        [Column("gender")]
        public string Gender { get; set; }

        [Column("age")]
        public int Age { get; set; }

        [Column("weight")]
        public float Weight { get; set; }

        [Column("address")]
        public string Address { get; set; }

        [Column("name")]
        public string Name { get; set; }

        public Pet() { }

        public Pet(Guid ownerId, Guid petId, string chipId, string species, string breed, string color, string gender, int age, float weight, string address, string name)
        {
            OwnerId = ownerId;
            PetId = petId;
            ChipId = chipId;
            Species = species;
            Breed = breed;
            Color = color;
            Gender = gender;
            Age = age;
            Weight = weight;
            Address = address;
            Name = name;
        }

        public static Pet Random(Guid ownerId)
        {
            var random = new Random();
            return new Pet(
                ownerId,
                Guid.NewGuid(),
                "",
                "",
                "",
                "",
                "",
                1 + random.Next(100),
                5.0f + 10.0f * (float)random.NextDouble(),
                "home",
                RandomString(8)
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
            return $"Pet{{ownerId={OwnerId}, petId={PetId}, chipId='{ChipId}', species='{Species}', breed='{Breed}', color='{Color}', gender='{Gender}', age={Age}, weight={Weight}, address='{Address}', name='{Name}'}}";
        }
    }
}
