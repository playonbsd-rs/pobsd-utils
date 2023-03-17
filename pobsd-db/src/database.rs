use paste::paste;
use pobsd_parser::Game;
use std::collections::HashMap;

macro_rules! add_game_to {
    ($field:ident) => {
        paste! {
            fn [<add_game_to_ $field>](&mut self, item: &str, game_id: u32) {
                match self.[<$field>].get_mut(item) {
                    Some(item) => item.push(game_id),
                    None => {
                        let _ = self.[<$field>].insert(item.into(), vec![game_id]);
                    }
                }
            }
        }
    };
}

macro_rules! push {
    ($game:ident, $field:ident, $db:ident ) => {
        paste! {
            if let Some(item) = &$game.$field {
                $db.[<add_game_to_ $field s>](&item, $game.uid);
            }
        }
    };
    ($game:ident, array $field:ident, $db:ident) => {
        paste! {
            if let Some(items) = &$game.$field {
                for item in items {
                    $db.[<add_game_to_ $field>](&item, $game.uid);
                }
            }
        }
    };
}

#[derive(Default)]
pub struct GameDataBase {
    pub(crate) games: HashMap<u32, Game>,
    pub(crate) engines: HashMap<String, Vec<u32>>,
    pub(crate) runtimes: HashMap<String, Vec<u32>>,
    pub(crate) genres: HashMap<String, Vec<u32>>,
    pub(crate) tags: HashMap<String, Vec<u32>>,
    pub(crate) years: HashMap<String, Vec<u32>>,
    pub(crate) devs: HashMap<String, Vec<u32>>,
    pub(crate) publis: HashMap<String, Vec<u32>>,
}

impl GameDataBase {
    pub fn new(games: Vec<Game>) -> Self {
        let mut db = GameDataBase::default();
        for game in games {
            db.load_game(game);
        }
        db
    }
    pub fn load_game(&mut self, game: Game) {
        let uid = game.uid;
        self.add_game(game);
        let game = self.games.get(&uid).unwrap().clone();

        push!(game, engine, self);
        push!(game, runtime, self);
        push!(game, array genres, self);
        push!(game, array tags, self);
        push!(game, year, self);
        push!(game, dev, self);
        push!(game, publi, self);
    }
    fn add_game(&mut self, game: Game) {
        self.games.insert(game.uid, game);
    }
    add_game_to!(tags);
    add_game_to!(engines);
    add_game_to!(runtimes);
    add_game_to!(genres);
    add_game_to!(years);
    add_game_to!(devs);
    add_game_to!(publis);
}

#[cfg(test)]
mod game_tests {
    use super::*;
    use pobsd_parser::Game;
    fn create_games() -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        let game1 = Game {
            uid: 1,
            name: "Game1".to_string(),
            cover: Some("Cover1".to_string()),
            engine: Some("Engine1".to_string()),
            setup: None,
            runtime: Some("Runtime1".to_string()),
            stores: Some(vec!["https://www.humblebundle.com/store/game1".to_string()]),
            hints: Some("Demo on HumbleBundle store page".to_string()),
            genres: Some(vec!["Genre1".to_string()]),
            tags: Some(vec!["Tag1".to_string()]),
            year: Some("2011".to_string()),
            dev: Some("Dev1".to_string()),
            publi: Some("Pub1".to_string()),
            version: None,
            status: None,
            added: Some("2022-01-02".to_string()),
            updated: None,
            igdb_id: None,
        };
        let game2 = Game {
            uid: 2,
            name: "Game2".to_string(),
            cover: Some("Cover2".to_string()),
            engine: Some("Engine2".to_string()),
            setup: None,
            runtime: Some("Runtime2".to_string()),
            stores: Some(vec!["https://www.humblebundle.com/store/game2".to_string()]),
            hints: Some("Demo on HumbleBundle store page".to_string()),
            genres: Some(vec!["Genre2".to_string()]),
            tags: Some(vec!["Tag2".to_string()]),
            year: Some("2011".to_string()),
            dev: Some("Dev2".to_string()),
            publi: Some("Pub2".to_string()),
            version: None,
            status: None,
            added: Some("2022-01-02".to_string()),
            updated: None,
            igdb_id: None,
        };
        games.push(game1);
        games.push(game2);
        games
    }
    #[test]
    fn test_database_get_all_tags() {
        let db = GameDataBase::new(create_games());
        let mut tags = db.get_all_tags();
        tags.items.sort();
        assert_eq!(tags.items, vec![&"Tag1".to_string(), &"Tag2".to_string()])
    }
    #[test]
    fn test_database_get_all_genres() {
        let db = GameDataBase::new(create_games());
        let mut genres = db.get_all_genres();
        genres.items.sort();
        assert_eq!(
            genres.items,
            vec![&"Genre1".to_string(), &"Genre2".to_string()]
        )
    }
    #[test]
    fn test_database_get_all_years() {
        let db = GameDataBase::new(create_games());
        let mut years = db.get_all_years();
        years.items.sort();
        assert_eq!(years.items, vec![&"2011".to_string()])
    }
    #[test]
    fn test_database_get_all_runtimes() {
        let db = GameDataBase::new(create_games());
        let mut runtimes = db.get_all_runtimes();
        runtimes.items.sort();
        assert_eq!(
            runtimes.items,
            vec![&"Runtime1".to_string(), &"Runtime2".to_string()]
        )
    }
    #[test]
    fn test_database_get_all_publis() {
        let db = GameDataBase::new(create_games());
        let mut publis = db.get_all_publis();
        publis.items.sort();
        assert_eq!(publis.items, vec![&"Pub1".to_string(), &"Pub2".to_string()])
    }
    #[test]
    fn test_database_get_all_devs() {
        let db = GameDataBase::new(create_games());
        let mut devs = db.get_all_devs();
        devs.items.sort();
        assert_eq!(devs.items, vec![&"Dev1".to_string(), &"Dev2".to_string()])
    }
    #[test]
    fn test_database_get_all_games() {
        let games_origin = create_games();
        let db = GameDataBase::new(create_games());
        let mut games = db.get_all_games();
        games.items.sort();
        assert_eq!(games.items[0], &games_origin[0]);
        assert_eq!(games.items[1], &games_origin[1]);
    }
}
