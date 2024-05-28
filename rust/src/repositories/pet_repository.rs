use std::sync::Arc;

use anyhow::{anyhow, Result};
use charybdis::types::Uuid;
use scylla::Session;

use crate::model::pet::Pet;

pub struct PetRepository {
    session: Arc<Session>,
}

impl PetRepository {
    pub async fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    pub async fn create(&self, pet: Pet) -> Result<()> {
        let query = "INSERT INTO pets (owner_id,pet_id) VALUES (?, ?)";
        let prepared = self.session.prepare(query).await?;

        self.session.execute(&prepared, (pet.owner_id, pet.pet_id)).await?;

        Ok(())
    }

    pub async fn list_by_owner_id(&self, id: Uuid, per_page: i32) -> Result<Vec<Pet>> {
        let query = "\
        SELECT \
            owner_id,
            pet_id,
            chip_id,
            species,
            breed,
            color,
            gender,
            age,
            weight,
            address,
            name
         FROM
            pets
         WHERE owner_id = ?
        ";

        let mut prepared = self.session.prepare(query).await?;
        prepared.set_page_size(per_page);

        let result = self.session.execute(&prepared, (id, )).await?;

        let pets = result.rows_typed::<Pet>()?.collect::<Result<Vec<_>, _>>()?;
        if pets.len() == 0 {
            return Err(anyhow!("Pet not found"));
        }

        Ok(pets)
    }
}
