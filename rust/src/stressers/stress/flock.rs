use std::collections::HashMap;

use anyhow::Result;
use log::*;
use rand::Rng;
use uuid::Uuid;

use crate::cli::StressConfig;
use crate::model::owner::Owner;
use crate::model::pet::Pet;
use crate::model::sensors::sensor::Sensor;
use crate::repositories::Repositories;

pub struct Flock {
    pub owners: Vec<Owner>,
    pub pets: Vec<Pet>,
    pub sensors: HashMap<Uuid, Vec<Sensor>>,
}

impl Flock {
    pub fn new(stress_config: &StressConfig) -> Flock {
        let owners = (0..stress_config.owners)
            .map(|_| Owner::random())
            .collect::<Vec<_>>();

        info!("Owners created");

        let mut rng = rand::thread_rng();

        let pets = (0..stress_config.pets)
            .map(|_| Pet::random(&owners[rng.gen_range(0..owners.len())]))
            .collect::<Vec<_>>();

        info!("Pets created");

        let sensors = pets
            .iter()
            .map(|pet| {
                let sensors = (0..rng.gen_range(1..stress_config.sensors))
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

    pub async fn save(&self, repositories: &Repositories) -> Result<()> {
        for owner in &self.owners {
            repositories.owner.create(owner.clone()).await?;
        }

        for pet in &self.pets {
            repositories.pet.create(pet.clone()).await?;
        }

        for sensors in self.sensors.values() {
            for sensor in sensors {
                repositories.sensor.create(sensor.clone()).await?;
            }
        }

        Ok(())
    }
}
