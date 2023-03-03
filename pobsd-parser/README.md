This crate provides a simplistic parser that converts
the [PlayOnBSD Database](https://github.com/playonbsd/OpenBSD-Games-Database)
(either provided as a string or as a file) into a vector of `Game` objects.

## Parser
A new parser can be create using the `Parser::new` method and providing
a `ParsingMode` enum as only argument.
The parsing supports two modes represented by the two variants of the
`ParsingMode` enum:
* **strict mode** (`ParsingMode::Strict`) in which the parsing
 will stop if a parsing error occurs returning the games processed
before the error as well as the line in the input (file or string)
where the error occured;
* **relaxed mode** (`ParsingMode::Relaxed`) where the parsing
will continue even after an error is encountered, the parsing
resuming when reaching the next game after the parsing error
it returns all the games that have been parsed as well as
the lines that were ignored due to parsing errors.

The database can be provided as a string using the `Parser::load_from_string` method
or as a file using the `Parser::load_from_file` method.

### Returned value
The returned value depend on the method used to parse the PlayOnBSD database.

The `Parser::load_from_string` method returns an `ParserResult` enum. It has to variants:
* `ParserResult::WithoutError` holding a vector of `Game` object;
* `ParserResult::WithError` holding a vector of `Game` objects as well as
a vector of [`usize`] where each element is the number of a line ignored during parsing
due to parsing errors.

The `Parser::load_from_file` method returns `Result<ParserResult, std::io::Error>`.
