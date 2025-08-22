using System;
using System.Collections.Generic;
using System.Linq;
using Microsoft.AspNetCore.Mvc;
using Microsoft.Extensions.Logging;
using Cassandra;
using CarePet.Model;
using System.Net;
using System.Threading.Tasks;

namespace CarePet.Server
{
    [ApiController]
    [Route("api")]
    public class ModelController : ControllerBase
    {
        private readonly ISession _session;
        private readonly Mapper _mapper;
        private readonly ILogger<ModelController> _logger;

        public ModelController(ISession session, ILogger<ModelController> logger)
        {
            _session = session;
            session.ChangeKeyspace(Config.Keyspace);
            _mapper = new Mapper(session);
            _logger = logger;
        }

        private static void GroupBy(List<float> data, List<Measure> measures, int startHour, DateTime day, DateTime nowUtc)
        {
            bool sameDate = nowUtc.Date == day.Date;
            int lastHour = nowUtc.Hour;

            var ag = new (double value, int total)?[24];

            foreach (var m in measures)
            {
                int hour = m.Ts.UtcDateTime.Hour;

                if (!ag[hour].HasValue)
                    ag[hour] = (0, 0);

                var entry = ag[hour].Value;
                entry.total++;
                entry.value += m.Value;
                ag[hour] = entry;
            }

            for (int hour = startHour; hour < 24; hour++)
            {
                if (!sameDate || hour <= lastHour)
                {
                    if (!ag[hour].HasValue)
                        ag[hour] = (0, 0);
                }
            }

            for (int hour = startHour; hour < ag.Length && ag[hour].HasValue; hour++)
            {
                var entry = ag[hour].Value;
                data.Add(entry.total > 0 ? (float)(entry.value / entry.total) : 0f);
            }
        }

        [HttpGet("owner/{id}")]
        public ActionResult<Owner> Owner(Guid id)
        {
            var owner = _mapper.Owner().Get(id);
            if (owner == null)
                return NotFound();
            return Ok(owner);
        }

        [HttpGet("owner/{id}/pets")]
        public ActionResult<IEnumerable<Pet>> Pets(Guid id)
        {
            return Ok(_mapper.Pet().FindByOwner(id));
        }

        [HttpGet("pet/{id}/sensors")]
        public ActionResult<IEnumerable<Sensor>> Sensors(Guid id)
        {
            return Ok(_mapper.Sensor().FindByPet(id));
        }

        [HttpGet("sensor/{id}/values")]
        public async Task<ActionResult<IEnumerable<float>>> Values(Guid id, [FromQuery] string from, [FromQuery] string to)
        {
            var resultSet = await _mapper.Measurement().FindAsync(id, DateTimeOffset.Parse(from).UtcDateTime, DateTimeOffset.Parse(to).UtcDateTime);
            var values = resultSet.Select(row => row).ToList();
            return Ok(values);
        }

        [HttpGet("sensor/{id}/values/day/{day}")]
        public async Task<ActionResult<IEnumerable<float>>> Avg(Guid id, string day)
        {
            var date = DateTime.Parse(day).Date;
            if (date > DateTime.UtcNow.Date)
            {
                return BadRequest("request into the future");
            }

            var resultSet = await _mapper.SensorAvg().FindAsync(id, date);
            var data = resultSet.Select(row => row).ToList();

            if (data.Count != 24)
            {
                data = new List<float>(data);
                Aggregate(id, date, data).GetAwaiter().GetResult();
            }

            return Ok(data);
        }

        public async System.Threading.Tasks.Task Aggregate(Guid id, DateTime day, List<float> data)
        {
            var nowUtc = DateTime.UtcNow;

            if (day > nowUtc.Date)
            {
                throw new ArgumentException("request into the future");
            }

            int startHour = data.Count;
            var startDate = new DateTimeOffset(day, TimeSpan.Zero);
            var endDate = new DateTimeOffset(day.AddHours(23).AddMinutes(59).AddSeconds(59).AddTicks(9999999), TimeSpan.Zero);

            var rows = await _mapper.Measurement()
                .FindWithTimestampsAsync(id, startDate.UtcDateTime, endDate.UtcDateTime);

            var measures = rows
                .Select(row => new Measure(id, row.Ts, row.Value))
                .ToList();

            int prevSize = data.Count;
            GroupBy(data, measures, startHour, day, nowUtc);
            SaveAggregate(id, data, prevSize, day, nowUtc);
        }

        private void SaveAggregate(Guid sensorId, List<float> data, int prevSize, DateTime day, DateTime nowUtc)
        {
            bool sameDate = nowUtc.Date == day.Date;
            int currentHour = nowUtc.Hour;

            for (int hour = prevSize; hour < data.Count; hour++)
            {
                if (sameDate && hour >= currentHour)
                    break;

                _mapper.SensorAvg().CreateAsync(new SensorAvg(sensorId, day, hour, data[hour]));
            }
        }

        [NonAction]
        public void Close()
        {
            _session.Dispose();
        }
    }
}
