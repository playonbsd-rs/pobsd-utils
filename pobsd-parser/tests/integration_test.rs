extern crate pobsd_parser;
use pobsd_parser::{Game, Parser, ParserResult};

#[test]
fn test_parsing_correct_database() {
    let games = match Parser::default()
        .load_from_file("tests/data/test-games.db")
        .unwrap()
    {
        ParserResult::WithoutError(games) => games,
        ParserResult::WithError(games, _) => {
            assert!(false);
            games
        }
    };
    // we get them all
    assert_eq!(games.len(), 9);
    // we get the right ones and in the right order
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
fn test_parsing_incorrect_database() {
    let (games, lines) = match Parser::default()
        .load_from_file("tests/data/test-games-faulty.db")
        .unwrap()
    {
        ParserResult::WithoutError(games) => {
            assert!(false);
            (games, vec![0])
        }
        ParserResult::WithError(games, lines) => (games, lines),
    };
    assert_eq!(vec![19, 48, 97], lines);
    // we get them all
    assert_eq!(games.len(), 9);
    // we get the right ones and in the right order
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
