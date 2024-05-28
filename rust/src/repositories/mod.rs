use std::sync::Arc;
use scylla::Session;
use crate::repositories::owner_repository::OwnerRepository;
use crate::repositories::pet_repository::PetRepository;
use crate::repositories::sensor_repository::SensorRepository;

pub mod owner_repository;
pub mod pet_repository;
pub mod sensor_repository;


pub struct Repositories {
    pub owner: OwnerRepository,
    pub pet: PetRepository,
    pub sensor: SensorRepository,
}

impl Repositories {
    pub async fn new (session: Arc<Session>) -> Repositories {
        Repositories {
            owner: OwnerRepository::new(session.clone()).await,
            pet: PetRepository::new(session.clone()).await,
            sensor: SensorRepository::new(session.clone()).await,
        }
    }
}
