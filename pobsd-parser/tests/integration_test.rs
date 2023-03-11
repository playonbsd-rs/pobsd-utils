extern crate pobsd_parser;
use pobsd_parser::{Game, Parser, ParserResult, ParsingMode};

// helper function to return the games with both
// correct and faulty database in relaxed mode
fn get_games(file: &str) -> Vec<Game> {
    match Parser::default()
        .load_from_file(file)
        .expect("Could not open the file")
    {
        ParserResult::WithoutError(games) => games,
        ParserResult::WithError(games, _) => games,
    }
}
// helper function to return the games with both
// correct and faulty database in relaxed mode
fn get_games_strict(file: &str) -> Vec<Game> {
    match Parser::new(ParsingMode::Strict)
        .load_from_file(file)
        .expect("Could not open the file")
    {
        ParserResult::WithoutError(games) => games,
        ParserResult::WithError(games, _) => games,
    }
}

#[test]
fn test_parser_return_value_correct_with_database_relaxed_mode() {
    match Parser::default()
        .load_from_file("tests/data/test-games.db")
        .unwrap()
    {
        ParserResult::WithoutError(_) => assert!(true),
        ParserResult::WithError(_, _) => assert!(false),
    };
}
#[test]
fn test_parser_return_value_correct_with_database_strict_mode() {
    match Parser::new(ParsingMode::Strict)
        .load_from_file("tests/data/test-games.db")
        .unwrap()
    {
        ParserResult::WithoutError(_) => assert!(true),
        ParserResult::WithError(_, _) => assert!(false),
    };
}
#[test]
fn test_parser_return_value_with_faulty_database_relaxed_mode() {
    match Parser::default()
        .load_from_file("tests/data/test-games-faulty.db")
        .unwrap()
    {
        ParserResult::WithoutError(_) => assert!(false),
        ParserResult::WithError(_, _) => assert!(true),
    };
}
#[test]
fn test_parser_return_value_with_faulty_database_strict_mode() {
    match Parser::default()
        .load_from_file("tests/data/test-games-faulty.db")
        .unwrap()
    {
        ParserResult::WithoutError(_) => assert!(false),
        ParserResult::WithError(_, _) => assert!(true),
    };
}
#[test]
fn test_parser_returned_lines_with_error_with_faulty_database_relaxed_mode() {
    let (_, lines) = match Parser::default()
        .load_from_file("tests/data/test-games-faulty.db")
        .unwrap()
    {
        ParserResult::WithoutError(games) => {
            assert!(false);
            (games, vec![0])
        }
        ParserResult::WithError(games, lines) => (games, lines),
    };
    assert_eq!(vec![20, 51, 97], lines);
}
#[test]
fn test_parser_returned_lines_with_error_with_faulty_database_strict_mode() {
    let (_, lines) = match Parser::new(ParsingMode::Strict)
        .load_from_file("tests/data/test-games-faulty.db")
        .unwrap()
    {
        ParserResult::WithoutError(games) => {
            assert!(false);
            (games, vec![0])
        }
        ParserResult::WithError(games, lines) => (games, lines),
    };
    assert_eq!(vec![20], lines);
}
#[test]
fn test_parser_right_number_of_games_with_correct_database_relaxed_mode() {
    let games = get_games("tests/data/test-games.db");
    // we get them all
    assert_eq!(games.len(), 9);
}
#[test]
fn test_parser_right_number_of_games_with_correct_database_strict_mode() {
    let games = get_games_strict("tests/data/test-games.db");
    // we get them all
    assert_eq!(games.len(), 9);
}
#[test]
fn test_parser_right_games_with_correct_database_relaxed_mode() {
    let games = get_games("tests/data/test-games.db");
    assert_eq!(
        games.get(0).unwrap().name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome"
    );
    assert_eq!(games.get(1).unwrap().name, "The Adventures of Mr. Hat");
    assert_eq!(games.get(2).unwrap().name, "The Adventures of Shuggy");
    assert_eq!(games.get(3).unwrap().name, "Aedemphia");
    assert_eq!(games.get(4).unwrap().name, "Aeternum");
    assert_eq!(games.get(5).unwrap().name, "Airships: Conquer the Skies");
    assert_eq!(games.get(6).unwrap().name, "Akane the Kunoichi");
    assert_eq!(games.get(7).unwrap().name, "Alien Shepherd");
    assert_eq!(games.get(8).unwrap().name, "Always Sometimes Monsters");
}
#[test]
fn test_parser_right_games_with_correct_database_strict_mode() {
    let games = get_games_strict("tests/data/test-games.db");
    assert_eq!(
        games.get(0).unwrap().name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome"
    );
    assert_eq!(games.get(1).unwrap().name, "The Adventures of Mr. Hat");
    assert_eq!(games.get(2).unwrap().name, "The Adventures of Shuggy");
    assert_eq!(games.get(3).unwrap().name, "Aedemphia");
    assert_eq!(games.get(4).unwrap().name, "Aeternum");
    assert_eq!(games.get(5).unwrap().name, "Airships: Conquer the Skies");
    assert_eq!(games.get(6).unwrap().name, "Akane the Kunoichi");
    assert_eq!(games.get(7).unwrap().name, "Alien Shepherd");
    assert_eq!(games.get(8).unwrap().name, "Always Sometimes Monsters");
}
#[test]
fn test_parser_right_number_of_games_with_faulty_database_relaxed_mode() {
    let games = get_games("tests/data/test-games-faulty.db");
    // we get them all
    assert_eq!(games.len(), 9);
}
#[test]
fn test_parser_right_number_of_games_with_faulty_database_strict_mode() {
    let games = get_games_strict("tests/data/test-games-faulty.db");
    // we get them all
    assert_eq!(games.len(), 2);
}
#[test]
fn test_parser_right_games_with_faulty_database_relaxed_mode() {
    let games = get_games("tests/data/test-games-faulty.db");
    assert_eq!(
        games.get(0).unwrap().name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome"
    );
    assert_eq!(games.get(1).unwrap().name, "The Adventures of Mr. Hat");
    assert_eq!(games.get(2).unwrap().name, "The Adventures of Shuggy");
    assert_eq!(games.get(3).unwrap().name, "Aedemphia");
    assert_eq!(games.get(4).unwrap().name, "Aeternum");
    assert_eq!(games.get(5).unwrap().name, "Airships: Conquer the Skies");
    assert_eq!(games.get(6).unwrap().name, "Akane the Kunoichi");
    assert_eq!(games.get(7).unwrap().name, "Alien Shepherd");
    assert_eq!(games.get(8).unwrap().name, "Always Sometimes Monsters");
}
#[test]
fn test_parser_right_games_with_faulty_database_strict_mode() {
    let games = get_games_strict("tests/data/test-games-faulty.db");
    assert_eq!(
        games.get(0).unwrap().name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome"
    );
    assert_eq!(games.get(1).unwrap().name, "The Adventures of Mr. Hat");
}
