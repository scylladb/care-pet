package main

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"time"

	"github.com/gocql/gocql"

	"github.com/scylladb/care-pet/go/db"
	"github.com/scylladb/care-pet/go/model"
	"github.com/scylladb/gocqlx/v2"
)

type pet struct {
	o *model.Owner
	p *model.Pet
	s []*model.Sensor
}

func NewPet() *pet {
	o := model.RandOwner()

	p := &pet{
		o: o,
		p: model.RandPet(o),
	}

	t := 1 + rand.Intn(len(model.SensorTypes))
	for i := 0; i < t; i++ {
		p.s = append(p.s, model.RandSensor(p.p))
	}

	return p
}

func (p *pet) save(ctx context.Context, s gocqlx.Session) error {
	if err := db.TableOwner.InsertQueryContext(ctx, s).BindStruct(p.o).ExecRelease(); err != nil {
		return fmt.Errorf("insert owner %s: %w", p.o.OwnerID.String(), err)
	}

	if err := db.TablePet.InsertQueryContext(ctx, s).BindStruct(p.p).ExecRelease(); err != nil {
		return fmt.Errorf("insert pet %s: %w", p.p.PetID.String(), err)
	}

	for _, sen := range p.s {
		if err := db.TableSensor.InsertQueryContext(ctx, s).BindStruct(sen).ExecRelease(); err != nil {
			return fmt.Errorf("insert sensor %s: %w", sen.SensorID.String(), err)
		}
	}

	return nil
}

func (p *pet) run(ctx context.Context, s gocqlx.Session) {
	if *verbose {
		log.Println("pet #", p.p.PetID, "ready")
	}

	var last = time.Now()
	for {
		var ms []*model.Measure

		for time.Since(last) < *bufferInterval {
			time.Sleep(*measure)

			for _, sen := range p.s {
				m := readSensorData(sen)
				ms = append(ms, m)

				log.Println("sensor #", sen.SensorID, "type", sen.Type, "new measure", m.Value, "ts", m.TS.Format(time.RFC3339))
			}
		}

		last = last.Add(*measure * (time.Since(last) / (*measure)))

		log.Println("pushing data")
		// this is simplified example of batch execution. standard
		// best practice is to push values that end up in the same partition:
		// https://www.scylladb.com/2019/03/27/best-practices-for-scylla-applications/
		b := s.NewBatch(gocql.UnloggedBatch)
		cql, _ := db.TableMeasure.Insert()

		for _, m := range ms {
			b.Query(cql, m.SensorID, m.TS, m.Value)
		}

		if err := s.ExecuteBatch(b); err != nil {
			log.Println("execute batch error: ", err)
			continue
		}

		ms = ms[:0]
	}
}

func readSensorData(sen *model.Sensor) *model.Measure {
	return &model.Measure{
		SensorID: sen.SensorID,
		TS:       time.Now(),
		Value:    model.RandSensorData(sen),
	}
}
