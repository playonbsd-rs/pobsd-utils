//! This library provides a set of methods to interogate the PlayOnBSD
//! database in a friendly manner, without having to deal with a SQL
//! database.
pub mod database;
pub(crate) mod queries;
pub mod query_result;

pub use database::GameDataBase;
pub use query_result::QueryResult;

type Item = String;
