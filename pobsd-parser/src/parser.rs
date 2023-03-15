//! This parser provides a simplistic [`Parser`] that converts
//! the [PlayOnBSD Database](https://github.com/playonbsd/OpenBSD-Games-Database)
//! (either provided as a string or as a file) into a vector of [`Game`] objects.
//!
use hash32::{FnvHasher, Hasher};
use std::fs;
use std::hash::Hash;
use std::path::Path;

use super::field::Field;
use super::game::Game;

pub trait State {}

enum ParserState {
    Game,
    Cover,
    Engine,
    Setup,
    Runtime,
    Store,
    Hints,
    Genre,
    Tags,
    Year,
    Dev,
    Pub,
    Version,
    Status,
    Added,
    Updated,
    IgdbId,
    Error,
    Recovering,
}

/// The [`ParsingMode`] enum is used to represent the two parsing modes
/// supported by [`Parser`]:
/// * a **strict mode** in which the parsing
///  will stop if a parsing error occurs returning the games processed
/// before the error as well as the line in the input (file or string)
/// where the error occured;
/// * a **relaxed mode** in which the parsing
/// will continue even after an error is encountered, the parsing
/// resuming when reaching the next game after the parsing error
/// ; it returns all the games that have been parsed as well as
/// the lines that were ignored due to parsing errors.
pub enum ParsingMode {
    Strict,
    Relaxed,
}

/// Represent the result of the parsing.
///
/// If there is no error, the [`ParserResult::WithoutError`] variant
/// is returned holding a vector of the games in the database. If there
/// is at least one error, the [`ParserResult::WithError`] variant is
/// returned holding a vector of the games in the database and a vector
/// of the lines where errors occured. If in strict mode only the games
/// parsed before the error occured will be returned.
pub enum ParserResult {
    WithError(Vec<Game>, Vec<usize>),
    WithoutError(Vec<Game>),
}

impl From<ParserResult> for Vec<Game> {
    fn from(val: ParserResult) -> Self {
        match val {
            ParserResult::WithError(games, _) => games,
            ParserResult::WithoutError(games) => games,
        }
    }
}
/// Parser provides a parser that can be created using the [`Parser::new`] method
/// which takes a [`ParsingMode`] enum as only argument.
pub struct Parser {
    state: ParserState,
    games: Vec<Game>,
    current_line: usize,
    error_lines: Vec<usize>,
    mode: ParsingMode,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
            current_line: 0,
            error_lines: Vec::new(),
            mode: ParsingMode::Relaxed,
        }
    }
}
impl Parser {
    /// Crate a parser with a given parsing mode
    pub fn new(mode: ParsingMode) -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
            current_line: 0,
            error_lines: Vec::new(),
            mode,
        }
    }
    /// Load the database from a file.
    pub fn load_from_file(self, file: impl AsRef<Path>) -> Result<ParserResult, std::io::Error> {
        let file: &Path = file.as_ref();
        if file.is_file() {
            let data = fs::read_to_string(file)?;
            Ok(self.load_from_string(&data))
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "This is not a file",
            ))
        }
    }
    /// Load the database from a string.
    pub fn load_from_string(mut self, data: &str) -> ParserResult {
        for line in data.lines() {
            self.current_line += 1;
            self.parse(line);
            if let ParserState::Error = self.state {
                self.error_lines.push(self.current_line);
                if let ParsingMode::Strict = self.mode {
                    break;
                }
            };
        }
        for game in &mut self.games {
            let mut fnv = FnvHasher::default();
            game.added.hash(&mut fnv);
            game.name.hash(&mut fnv);
            game.uid = fnv.finish32();
        }
        match self.error_lines.is_empty() {
            false => ParserResult::WithError(self.games, self.error_lines),
            true => ParserResult::WithoutError(self.games),
        }
    }
    impl_parse![ParserState::Game, Field::Game, name, ParserState::Cover;
         (ParserState::Cover, Field::Cover, cover, ParserState::Engine);
         (ParserState::Engine, Field::Engine, engine, ParserState::Setup);
         (ParserState::Setup, Field::Setup, setup, ParserState::Runtime);
         (ParserState::Runtime, Field::Runtime, runtime, ParserState::Store);
         (ParserState::Store, Field::Store, stores, ParserState::Hints);
         (ParserState::Hints, Field::Hints, hints, ParserState::Genre);
         (ParserState::Genre, Field::Genres, genres, ParserState::Tags);
         (ParserState::Tags, Field::Tags, tags, ParserState::Year);
         (ParserState::Year, Field::Year, year, ParserState::Dev);
         (ParserState::Dev, Field::Dev, dev, ParserState::Pub);
         (ParserState::Pub, Field::Publi, publi, ParserState::Version);
         (ParserState::Version, Field::Version, version, ParserState::Status);
         (ParserState::Status, Field::Status, status, ParserState::Added);
         (ParserState::Added, Field::Added, added, ParserState::Updated);
         (ParserState::Updated, Field::Updated, updated, ParserState::IgdbId);
         (ParserState::IgdbId, Field::IgdbId, igdb_id, ParserState::Game)
    ];
}
