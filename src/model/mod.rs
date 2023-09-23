#![allow(dead_code)]

use sqlite::Connection;
pub enum DbConections {
    UraLoader,
}
impl DbConections {
    pub fn open(self) -> Result<Connection, sqlite::Error> {
        match self {
            Self::UraLoader => sqlite::open("db/ura_loader.sqlite"),
        }
    }
}
pub mod user;