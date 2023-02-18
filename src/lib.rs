use crate::parser::{Parser, ParserResult};
use std::path::Path;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
pub(crate) mod db;
pub(crate) mod parser;

pub fn check(file: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let parser = Parser::default();
    match parser.load_from_file(&file)? {
        ParserResult::WithError(games, lines) => {
            let message: Vec<String> = lines.into_iter().map(|x| x.to_string()).collect();
            println!("> {} games parsed.", games.len());
            println!("> Error occured at lines {}.", message.join(", "));
        }
        ParserResult::WithoutError(games) => {
            println!("> {} games parsed without error.", games.len());
        }
    }
    Ok(())
}
