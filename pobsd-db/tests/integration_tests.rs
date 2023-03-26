extern crate pobsd_db;
extern crate pobsd_parser;
use pobsd_db::GameDataBase;
use pobsd_parser::{Game, Parser, ParserResult, ParsingMode};

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

fn get_db_strict() -> GameDataBase {
    let games = get_games_strict("tests/data/test-games.db");
    GameDataBase::new(games)
}

#[test]
fn test_get_by_tag() {
    let db = get_db_strict();
    let game_query = db.get_game_by_tag("indie");
    assert_eq!(game_query.items.len(), 3);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
    let game = game_query.items.get(1).unwrap();
    assert_eq!(game.name, "The Adventures of Shuggy".to_string());
    let game = game_query.items.get(2).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}

#[test]
fn test_get_by_year() {
    let db = get_db_strict();
    let game_query = db.get_game_by_year("2011");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(
        game.name,
        "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string()
    );
}

#[test]
fn test_get_by_engine() {
    let db = get_db_strict();
    let game_query = db.get_game_by_engine("godot");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "The Adventures of Mr. Hat".to_string());
}

#[test]
fn test_get_by_runtime() {
    let db = get_db_strict();
    let game_query = db.get_game_by_runtime("lwjgl");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Airships: Conquer the Skies".to_string());
}

#[test]
fn test_get_by_genre() {
    let db = get_db_strict();
    let game_query = db.get_game_by_genre("shmup");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}

#[test]
fn test_get_by_dev() {
    let db = get_db_strict();
    let game_query = db.get_game_by_dev("Creaky Lantern Games");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Aeternum".to_string());
}

#[test]
fn test_get_by_publi() {
    let db = get_db_strict();
    let game_query = db.get_game_by_publi("Florent Espanet");
    assert_eq!(game_query.items.len(), 1);
    let game = game_query.items.get(0).unwrap();
    assert_eq!(game.name, "Alien Shepherd".to_string());
}

#[test]
fn test_get_all_tags() {
    let db = get_db_strict();
    let tag_query = db.get_all_tags();
    assert_eq!(tag_query.items.len(), 6);
    for tag in vec!["Tag1", "indie", "free", "manga", "bullethell", "anime"] {
        assert!(tag_query.items.contains(&&tag.to_string()));
    }
}

#[test]
fn test_get_all_genres() {
    let db = get_db_strict();
    let tag_query = db.get_all_genres();
    assert_eq!(tag_query.items.len(), 7);
    for genre in vec![
        "Genre1",
        "Puzzle Platformer",
        "RPG",
        "shmup",
        "RTS",
        "Platformer",
        "platformer",
    ] {
        assert!(tag_query.items.contains(&&genre.to_string()));
    }
}

#[test]
fn test_get_all_runtimes() {
    let db = get_db_strict();
    let tag_query = db.get_all_runtimes();
    assert_eq!(tag_query.items.len(), 7);
    for runtime in vec![
        "godot",
        "easyrpg",
        "fnaify",
        "lwjgl",
        "HashLink",
        "HTML5",
        "HumblePlay",
    ] {
        assert!(tag_query.items.contains(&&runtime.to_string()));
    }
}
