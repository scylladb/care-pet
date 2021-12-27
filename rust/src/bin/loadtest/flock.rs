use std::collections::HashMap;

use anyhow::anyhow;
use log::*;
use rand::Rng;
use scylla::Session;
use uuid::Uuid;

use care_pet::db::{TABLE_OWNER, TABLE_PET, TABLE_SENSOR};
use care_pet::insert_query;
use care_pet::{Owner, Pet, Result, Sensor};

pub type Flock = (Vec<Owner>, Vec<Pet>, HashMap<Uuid, Vec<Sensor>>);

pub fn new(owners_count: usize, pets_count: usize, sensors_count: usize) -> Flock {
    let owners = (0..owners_count)
        .map(|_| Owner::random())
        .collect::<Vec<_>>();

    info!("Owners created");

    let mut rng = rand::thread_rng();

    let (pets, sensors) = (0..pets_count).fold(
        (Vec::with_capacity(pets_count), HashMap::new()),
        |(mut pets, mut sensors_map), _| {
            let pet = Pet::random(&owners[rng.gen_range(0..owners.len())]);

            let sensors = (0..rng.gen_range(1..sensors_count))
                .map(|_| Sensor::random(&pet))
                .collect::<Vec<_>>();

            sensors_map.insert(pet.pet_id, sensors);
            pets.push(pet);

            (pets, sensors_map)
        },
    );

    info!("Pets created");

    (owners, pets, sensors)
}

pub async fn save(sess: &Session, f: &Flock) -> Result<()> {
    for owner in &f.0 {
        sess.query(insert_query!(TABLE_OWNER, Owner), owner)
            .await
            .map(|_| ())
            .map_err(|err| anyhow!("insert owner {}: {:?}", owner.owner_id, err))?;
    }

    for pet in &f.1 {
        sess.query(insert_query!(TABLE_PET, Pet), pet)
            .await
            .map(|_| ())
            .map_err(|err| anyhow!("insert pet {}: {:?}", pet.pet_id, err))?;
    }

    for sensors in f.2.values() {
        for sensor in sensors {
            sess.query(insert_query!(TABLE_SENSOR, Sensor), sensor)
                .await
                .map(|_| ())
                .map_err(|err| anyhow!("insert sensor {}: {:?}", sensor.sensor_id, err))?;
        }
    }

    Ok(())
}
