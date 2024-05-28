use fake::Fake;
use fake::faker::name::raw::Name;
use fake::locales::EN;
use rand::Rng;
use scylla::{FromRow, ValueList};
use serde::Serialize;
use uuid::Uuid;


use crate::model::owner::Owner;

#[derive(Debug, Default, Clone, ValueList, FromRow, Serialize)]
pub struct Pet {
    pub owner_id: Uuid,
    pub pet_id: Uuid,
    pub chip_id: Option<String>,
    pub species: Option<String>,
    pub breed: Option<String>,
    pub color: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
    pub weight: Option<f32>,
    pub address: Option<String>,
    pub name: Option<String>,
}

impl Pet {
    pub fn random(o: &Owner) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            owner_id: o.owner_id,
            pet_id: Uuid::new_v4(),
            age: Option::from(rng.gen_range(1..100)),
            weight: Option::from(rng.gen_range(5.0..10.0)),
            address: Option::from(o.address.clone()),
            name: Name(EN).fake(),
            ..Default::default()
        }
    }
}
