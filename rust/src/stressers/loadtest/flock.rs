use std::collections::HashMap;

use anyhow::anyhow;
use log::*;
use rand::Rng;
use scylla::Session;
use uuid::Uuid;

use care_pet::insert_query;
use care_pet::{Owner, Pet, Result, Sensor};

pub struct Flock {
    pub owners: Vec<Owner>,
    pub pets: Vec<Pet>,
    pub sensors: HashMap<Uuid, Vec<Sensor>>,
}

impl Flock {
    pub fn new(owners_count: usize, pets_count: usize, sensors_count: usize) -> Flock {
        let owners = (0..owners_count)
            .map(|_| Owner::random())
            .collect::<Vec<_>>();

        info!("Owners created");

        let mut rng = rand::thread_rng();

        let pets = (0..pets_count)
            .map(|_| Pet::random(&owners[rng.gen_range(0..owners.len())]))
            .collect::<Vec<_>>();

        info!("Pets created");

        let sensors = pets
            .iter()
            .map(|pet| {
                let sensors = (0..rng.gen_range(1..sensors_count))
                    .map(|_| Sensor::random(pet))
                    .collect::<Vec<_>>();

                (pet.pet_id, sensors)
            })
            .collect::<HashMap<_, _>>();

        info!("Sensors created");

        Flock {
            owners,
            pets,
            sensors,
        }
    }

    pub async fn save(&self, sess: &Session) -> Result<()> {
        for owner in &self.owners {
            sess.query(insert_query!(Owner), owner)
                .await
                .map(|_| ())
                .map_err(|err| anyhow!("insert owner {}: {:?}", owner.owner_id, err))?;
        }

        for pet in &self.pets {
            sess.query(insert_query!(Pet), pet)
                .await
                .map(|_| ())
                .map_err(|err| anyhow!("insert pet {}: {:?}", pet.pet_id, err))?;
        }

        for sensors in self.sensors.values() {
            for sensor in sensors {
                sess.query(insert_query!(Sensor), sensor)
                    .await
                    .map(|_| ())
                    .map_err(|err| anyhow!("insert sensor {}: {:?}", sensor.sensor_id, err))?;
            }
        }

        Ok(())
    }
}
