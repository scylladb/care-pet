using System;
using Cassandra;

namespace CarePet.Model
{
    public class Mapper
    {
        private readonly ISession _session;

        public Mapper(ISession session)
        {
            _session = session ?? throw new ArgumentNullException(nameof(session));
        }

        public OwnerDAO Owner() => new OwnerDAO(_session);
        public PetDAO Pet() => new PetDAO(_session);
        public SensorDAO Sensor() => new SensorDAO(_session);
        public MeasureDAO Measurement() => new MeasureDAO(_session);
        public SensorAvgDAO SensorAvg() => new SensorAvgDAO(_session);

        public static MapperBuilder Builder(ISession session) => new MapperBuilder(session);
    }

    public class MapperBuilder
    {
        private readonly ISession _session;

        public MapperBuilder(ISession session)
        {
            _session = session;
        }

        public Mapper Build()
        {
            _session.ChangeKeyspace(Config.Keyspace);
            return new Mapper(_session);
        }
    }
}
