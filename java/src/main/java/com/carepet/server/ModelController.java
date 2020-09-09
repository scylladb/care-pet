package com.carepet.server;

import com.carepet.model.Mapper;
import com.carepet.model.Owner;
import com.carepet.model.Pet;
import com.carepet.model.Sensor;
import com.datastax.oss.driver.api.core.CqlSession;
import com.datastax.oss.driver.api.core.cql.ResultSet;
import io.micronaut.http.MediaType;
import io.micronaut.http.annotation.Controller;
import io.micronaut.http.annotation.Get;
import io.micronaut.http.annotation.QueryValue;
import io.micronaut.validation.Validated;
import io.reactivex.Observable;
import io.reactivex.Single;

import javax.inject.Inject;
import javax.validation.constraints.NotBlank;
import java.time.Instant;
import java.time.LocalDate;
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

    @Get(uri = "/sensor/{id}/values/day/{date}", produces = MediaType.APPLICATION_JSON)
    public Observable<Float> avg(@NotBlank UUID id, @NotBlank LocalDate date) {
        throw new UnsupportedOperationException("to implement");
    }

    public void close() {
        session.close();
    }
}
