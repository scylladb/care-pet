use std::sync::Arc;
use scylla::Session;


pub mod database;
pub mod log;
pub mod model;
pub mod repositories;
pub mod cli;
pub mod http;

pub mod stressors;

pub struct AppState {
    pub session: Arc<Session>,
}