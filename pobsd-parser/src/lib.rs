#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
pub mod parser_macros;
pub mod field;
pub mod game;
pub mod parser;
pub(crate) mod split_line;

pub use self::game::Game;
pub use self::parser::Parser;
pub use self::parser::ParserResult;
