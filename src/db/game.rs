use rusqlite::{Connection, Result};

#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct GameDb {
    pub id: i32,
    pub name: String,
    pub cover: Option<String>,
    pub setup: Option<String>,
    pub hints: Option<String>,
    pub year: Option<String>,
    pub version: Option<String>,
    pub status: Option<String>,
    pub added: Option<String>,
    pub updated: Option<String>,
}
