pub use crate::commands::browse::browse;
use pobsd_parser::{Game, Parser, ParserResult};
use serde_derive::Serialize;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize)]
struct GamesExport {
    count: usize,
    games: Vec<Game>,
}

impl GamesExport {
    pub fn new(games: Vec<Game>) -> Self {
        Self {
            count: games.len(),
            games,
        }
    }
}
pub fn check(db: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let parser = Parser::default();
    match parser.load_from_file(&db)? {
        ParserResult::WithError(games, lines) => {
            let message: Vec<String> = lines.into_iter().map(|x| x.to_string()).collect();
            println!("> {} games parsed.", games.len());
            println!("> Errors occured at lines {}.", message.join(", "));
        }
        ParserResult::WithoutError(games) => {
            println!("> {} games parsed without error.", games.len());
        }
    }
    Ok(())
}

pub fn export(db: impl AsRef<Path>, js: impl AsRef<Path>) -> Result<(), std::io::Error> {
    let parser = Parser::default();
    match parser.load_from_file(&db)? {
        ParserResult::WithError(_, lines) => {
            let message: Vec<String> = lines.into_iter().map(|x| x.to_string()).collect();
            eprintln!("> Errors occured at lines {}.", message.join(", "));
            eprintln!("> Export aborted.");
        }
        ParserResult::WithoutError(games) => {
            let game_export = GamesExport::new(games);
            let mut js = std::fs::File::create(js)?;
            js.write_all(&serde_json::to_vec_pretty(&game_export).unwrap())?;
        }
    }
    Ok(())
}
