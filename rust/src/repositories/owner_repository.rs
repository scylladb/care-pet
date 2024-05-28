use std::sync::Arc;

use anyhow::{anyhow, Result};
use scylla::Session;
use uuid::Uuid;

use crate::model::owner::Owner;

pub struct OwnerRepository {
    session: Arc<Session>,
}

impl OwnerRepository {
    pub async fn new(session: Arc<Session>) -> Self {
        Self { session }
    }

    pub async fn create(&self, owner: Owner) -> Result<()> {
        let query = "INSERT INTO owners (owner_id, name, address) VALUES (?, ?, ?)";
        let prepared = self.session.prepare(query).await?;

        self.session
            .execute(&prepared, (owner.owner_id, owner.name, owner.address))
            .await?;

        Ok(())
    }

    pub async fn find(&self, id: Uuid) -> Result<Owner> {
        let query = "SELECT owner_id, name, address FROM owners WHERE owner_id = ?";

        let mut prepared = self.session.prepare(query).await?;
        prepared.set_page_size(1);

        let result = self.session.execute(&prepared, (id, )).await?
            .rows_typed::<Owner>()?;


        if let Some(owner) = result.into_iter().next().transpose()? {
            return Ok(owner);
        }

        Err(anyhow!("Owner not found"))
    }
}
