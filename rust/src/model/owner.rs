use fake::Fake;
use fake::faker::name::fr_fr::Name;
use scylla::{FromRow, ValueList};
use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, ValueList, FromRow, Serialize,Debug)]
pub struct Owner {
    pub owner_id: Uuid,
    pub name: String,
    pub address: String,
}

impl Owner {
    pub fn random() -> Self {
        Self {
            owner_id: Uuid::new_v4(),
            name: Name().fake(),
            address: "1234 Fake St".to_string(),
        }
    }
}
