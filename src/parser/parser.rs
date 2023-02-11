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
