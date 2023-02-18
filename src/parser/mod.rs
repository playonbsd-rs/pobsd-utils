#[macro_use]
pub(crate) mod parser_macros;
pub(crate) mod field;
pub(crate) mod game;
pub(crate) mod parser;
pub(crate) mod split_line;

pub(crate) use self::parser::Parser;
pub(crate) use self::parser::ParserResult;
