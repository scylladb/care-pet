use std::sync::Arc;

use scylla::Session;

pub use crate::model::*;
pub use crate::result::Result;

pub mod db;
pub mod controllers;
pub mod log;
pub mod model;
pub mod result;

pub mod repositories;

pub struct AppState {
    pub session: Arc<Session>,
}