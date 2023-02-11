//! This parser provides a simplistic [`Parser`] that converts
//! the [PlayOnBSD Database](https://github.com/playonbsd/OpenBSD-Games-Database)
//! (either provided as a string or as a file) into a vector of [`Game`] objects.
//!
//! # Parser
//! A new parser can be create using the [`Parser::new`] method and proving
//! a [`ParsingMode`] enum as only argument.
//! The parsing supports two modes representend by the two variants of the
//! [`ParsingMode`] enum:
//! * **strict mode** ([`ParsingMode::Strict`]) in which the parsing
//!  will stop if a parsing error occurs returning the games processed
//! before the error as well as the line in the input (file or string)
//! where the error occured;
//! * **relaxed mode** ([`ParsingMode::Relaxed`]) where the parsing
//! will continue even after an error is encountered, the parsing
//! resuming when reaching the next game after the parsing error
//! ; it returns all the games that have been parsed as well as
//! the lines that were ignored due to parsing errors.
//!
//! The database can be provided as a string using the [`Parser::load_from_string`] method
//! or as a file using the [`Parser::load_from_file`] method.
//!
//! ### Returned value
//! The returned value depend on the method used to parse the PlayOnBSD database.
//!
//! The [`Parser::load_from_string`] method returns an [`ParserResult`] enum. It has to variants:
//! * [`ParserResult::WithoutError`] holding a vector of [`Game`] object;
//! * [`ParserResult::WithError`] holding a vector of [`Game`] objects as well as
//! a vector of [`usize`] where each element is the number of a line ignored during parsing
//! due to parsing errors.
//!
//! The [`Parser::load_from_file`] method returns [`Result`]<[`ParserResult`], [`std::io::Error`]>.
//!
//! ### Example
//!
//! ```no_run
//! use parser::{Parser, ParserResult};
//!
//! // Print the database
//! fn main() -> Result<(), std::io::Error> {
//!     let result = Parser::default().load_from_file("tests/data/good-data.db")?;
//!     match result {
//!         ParserResult::WithoutError(games) => {
//!             for game in games {
//!                 println!("{}", game);
//!             }
//!         },
//!         ParserResult::WithError(_, _) => println!("Parsing errors"),
//!     }
//!     Ok(())
//! }
//! ```
use std::fs;
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
    Error,
    Recovering,
}

pub enum ParsingMode {
    Strict,
    Relaxed,
}

pub enum ParserResult {
    WithError(Vec<Game>, Vec<usize>),
    WithoutError(Vec<Game>),
}

impl Into<Vec<Game>> for ParserResult {
    fn into(self) -> Vec<Game> {
        match self {
            ParserResult::WithError(games, _) => games,
            ParserResult::WithoutError(games) => games,
        }
    }
}

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
    pub fn new(mode: ParsingMode) -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
            current_line: 0,
            error_lines: Vec::new(),
            mode,
        }
    }
    pub fn load_from_file(self, file: impl AsRef<Path>) -> Result<ParserResult, std::io::Error> {
        let file: &Path = file.as_ref();
        if file.is_file() {
            let data = fs::read_to_string(file)?;
            Ok(self.load_from_string(&data))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "This is not a file"))
        }
    }
    pub fn load_from_string(mut self, data: &str) -> ParserResult {
        for line in data.lines() {
            self.current_line += 1;
            self.parse(line);
            if let ParserState::Error = self.state {
                self.error_lines.push(self.current_line);
                if let ParsingMode::Strict = self.mode {
                    break
                }
            };
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
         (ParserState::Updated, Field::Updated, updated, ParserState::Game)
    ];
}
