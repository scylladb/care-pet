using System;
using System.Threading.Tasks;
using Cassandra;
using Cassandra.Mapping;
using System.Collections.Generic;

namespace CarePet.Model
{
    public interface ISensorAvgDAO
    {
        Task CreateAsync(SensorAvg avg);
        Task UpdateAsync(SensorAvg avg);
        Task<SensorAvg> GetAsync(Guid sensorId, DateTime date, int hour);
        Task<IEnumerable<float>> FindAsync(Guid sensorId, DateTime date);
    }

    public class SensorAvgDAO : ISensorAvgDAO
    {
        private readonly IMapper _mapper;
        private readonly ISession _session;

        public SensorAvgDAO(ISession session)
        {
            _session = session;
            _mapper = new Cassandra.Mapping.Mapper(session);
        }

        public Task CreateAsync(SensorAvg avg)
        {
            return _mapper.InsertAsync(avg);
        }

        public Task UpdateAsync(SensorAvg avg)
        {
            return _mapper.UpdateAsync(avg);
        }

        public Task<SensorAvg> GetAsync(Guid sensorId, DateTime date, int hour)
        {
            return _mapper.FirstOrDefaultAsync<SensorAvg>(
                "WHERE sensor_id = ? AND date = ? AND hour = ?",
                sensorId, date, hour
            );
        }

        public async Task<IEnumerable<float>> FindAsync(Guid sensorId, DateTime date)
        {
            var rs = await _session.ExecuteAsync(new SimpleStatement(
                "SELECT value FROM sensor_avg WHERE sensor_id = ? AND date = ?",
                sensorId, date
            )).ConfigureAwait(false);

            var values = new List<float>();
            foreach (var row in rs)
            {
                values.Add(row.GetValue<float>("value"));
            }
            return values;
        }
    }
}
