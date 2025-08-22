using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Cassandra;
using Cassandra.Mapping;

namespace CarePet.Model
{
    public interface IMeasureDAO
    {
        Task CreateAsync(Measure measure);
        Task UpdateAsync(Measure measure);
        Task<Measure> GetAsync(Guid sensorId, DateTimeOffset timestamp);
        Task<IEnumerable<float>> FindAsync(Guid sensorId, DateTimeOffset start, DateTimeOffset end);
        Task<IEnumerable<(DateTimeOffset Ts, float Value)>> FindWithTimestampsAsync(Guid sensorId, DateTimeOffset start, DateTimeOffset end);
    }

    public class MeasureDAO : IMeasureDAO
    {
        private readonly IMapper _mapper;
        private readonly ISession _session;

        public MeasureDAO(ISession session)
        {
            _session = session;
            _mapper = new Cassandra.Mapping.Mapper(session);
        }

        public Task CreateAsync(Measure measure)
        {
            return _mapper.InsertAsync(measure);
        }

        public Task UpdateAsync(Measure measure)
        {
            return _mapper.UpdateAsync(measure);
        }

        public Task<Measure> GetAsync(Guid sensorId, DateTimeOffset timestamp)
        {
            return _mapper.FirstOrDefaultAsync<Measure>(
                "WHERE sensor_id = ? AND ts = ?",
                sensorId, timestamp
            );
        }

        public async Task<IEnumerable<float>> FindAsync(Guid sensorId, DateTimeOffset start, DateTimeOffset end)
        {
            var rs = await _session.ExecuteAsync(new SimpleStatement(
                "SELECT value FROM measurement WHERE sensor_id = ? AND ts >= ? AND ts <= ?",
                sensorId, start, end
            )).ConfigureAwait(false);

            var values = new List<float>();
            foreach (var row in rs)
            {
                values.Add(row.GetValue<float>("value"));
            }
            return values;
        }

        public async Task<IEnumerable<(DateTimeOffset Ts, float Value)>> FindWithTimestampsAsync(Guid sensorId, DateTimeOffset start, DateTimeOffset end)
        {
            var rs = await _session.ExecuteAsync(new SimpleStatement(
                "SELECT ts, value FROM measurement WHERE sensor_id = ? AND ts >= ? AND ts <= ?",
                sensorId, start, end
            )).ConfigureAwait(false);

            var results = new List<(DateTimeOffset, float)>();
            foreach (var row in rs)
            {
                var ts = row.GetValue<DateTimeOffset>("ts");
                var value = row.GetValue<float>("value");
                results.Add((ts, value));
            }
            return results;
        }
    }
}
