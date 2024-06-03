const DB_PATH: &str = "./data/db.json";

use std::io;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

//#[derive(Debug, Serialize, Deserialize, Clone)]
//pub struct Pet {
//    pub id: usize,
//    pub name: String,
//    pub category: String,
//    pub age: usize,
//    pub created_at: DateTime<Utc>,
//}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),

    #[error("Error parsing the DB file:{0}")]
    ParseDBError(#[from] serde_json::Error),
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Default, Debug)]
pub enum MenuItem {
    #[default]
    Home,
    Users,
}

impl From<MenuItem> for usize {
    fn from(value: MenuItem) -> Self {
        match value {
            MenuItem::Home => 0,
            MenuItem::Users => 1,
        }
    }
}
