package com.carepet.server;

import com.carepet.model.*;
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.ResultSet;
import io.micronaut.http.HttpStatus;
import io.micronaut.http.MediaType;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.QueryValue;
import io.micronaut.http.exceptions.HttpStatusException;
import io.micronaut.validation.Validated;
import io.reactivex.Observable;
import io.reactivex.Single;

import javax.inject.Inject;
import javax.validation.constraints.NotBlank;
import java.time.*;
import java.util.ArrayList;
import java.util.List;
import java.util.UUID;

@Controller("/api")
@Validated
public class ModelController {
    private CqlSession session;
    private Mapper mapper;

    @Inject
    public ModelController(CqlSession session) {
        this.session = session;
        this.mapper = Mapper.builder(session).build();
    }

    private static void groupBy(List<Float> data, List<Measure> measures, int startHour, LocalDate day, LocalDateTime now) {
        // if it's the same day, we can't aggregate current hour
        boolean sameDate = now.getDayOfYear() == day.getDayOfYear();
        int last = now.getHour();

        class Avg {
            double value;
            int total;
        }

        // aggregate data
        Avg[] ag = new Avg[24];

        for (Measure m : measures) {
            int hour = m.getTs().atOffset(ZoneOffset.UTC).getHour();

            if (ag[hour] == null) {
                ag[hour] = new Avg();
            }

            Avg a = ag[hour];
            a.total++;
            a.value += m.getValue();
        }

        // ensure data completeness
        for (int hour = startHour; hour < 24; hour++) {
            if (!sameDate || hour <= last) {
                if (ag[hour] == null) {
                    ag[hour] = new Avg();
                }
            }
        }

        // fill the avg
        for (int hour = startHour; hour < ag.length && ag[hour] != null; hour++) {
            Avg a = ag[hour];
            if (a.total > 0) {
                data.add((float) (a.value / a.total));
            } else {
                data.add(0.0f);
            }
        }
    }

    @Get(uri = "/owner/{id}", produces = MediaType.APPLICATION_JSON)
    public Single<Owner> owner(@NotBlank UUID id) {
        return Single.just(mapper.owner().get(id));
    }

    @Get(uri = "/owner/{id}/pets", produces = MediaType.APPLICATION_JSON)
    public Observable<Pet> pets(@NotBlank UUID id) {
        return Observable.fromIterable(mapper.pet().findByOwner(id));
    }

    @Get(uri = "/pet/{id}/sensors", produces = MediaType.APPLICATION_JSON)
    public Observable<Sensor> sensors(@NotBlank UUID id) {
        return Observable.fromIterable(mapper.sensor().findByPet(id));
    }

    @Get(uri = "/sensor/{id}/values", produces = MediaType.APPLICATION_JSON)
    public Observable<Float> values(@NotBlank UUID id, @NotBlank @QueryValue String from, @NotBlank @QueryValue String to) {
        ResultSet res = mapper.measurement().find(id, Instant.parse(from), Instant.parse(to));
        return Observable.fromIterable(res.map(x -> x.getFloat(0)));
    }

    @Get(uri = "/sensor/{id}/values/day/{day}", produces = MediaType.APPLICATION_JSON)
    public Observable<Float> avg(@NotBlank UUID id, @NotBlank String day) {
        LocalDate date = LocalDate.parse(day);
        if (date.isAfter(LocalDate.now())) {
            throw new HttpStatusException(HttpStatus.BAD_REQUEST, "request into the future");
        }

        ResultSet res = mapper.sensorAvg().find(id, date);
        List<Float> data = res.map(x -> x.getFloat(0)).all();

        if (data.size() != 24) {
            data = new ArrayList<>(data);
            aggregate(id, date, data);
        }

        return Observable.fromIterable(data);
    }

    public void aggregate(UUID id, LocalDate day, List<Float> data) {
        LocalDateTime now = LocalDateTime.now(Clock.systemUTC());

        // can't aggregate data for post today's date
        if (day.getDayOfYear() > now.getDayOfYear()) {
            throw new HttpStatusException(HttpStatus.BAD_REQUEST, "request into the future");
        }

        // we can start from next missing hour. hours = [0, 23]. len = [0, 24]
        int startHour = data.size();
        Instant startDate = day.atStartOfDay().toInstant(ZoneOffset.UTC);
        Instant endDate = day.atTime(23, 59, 59, 999999999).toInstant(ZoneOffset.UTC);

        List<Measure> measures = mapper.measurement().findWithTimestamps(id, startDate, endDate).
                map(row -> new Measure(null, row.getInstant(0), row.getFloat(1))).
                all();

        int prevAvgSize = data.size();
        groupBy(data, measures, startHour, day, now);

        saveAggregate(id, data, prevAvgSize, day, now);
    }

    // saveAggregate saves the result monotonically sequentially to the database
    private void saveAggregate(UUID sensorId, List<Float> data, int prevAvgSize, LocalDate day, LocalDateTime now) {
        // if it's the same day, we can't aggregate current hour
        boolean sameDate = now.getDayOfYear() == day.getDayOfYear();
        int current = now.getHour();

        for (int hour = prevAvgSize; hour < data.size(); hour++) {
            if (sameDate && hour >= current) {
                break;
            }

            mapper.sensorAvg().create(new SensorAvg(sensorId, day, hour, data.get(hour)));
        }
    }


    public void close() {
        session.close();
    }
}
