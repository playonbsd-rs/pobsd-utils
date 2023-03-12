use paste::paste;
use pobsd_parser::Game;

use crate::database::GameDataBase;

macro_rules! get_game_by {
    (id) => {
        pub fn get_game_by_id(&self, game_id: u32) -> Option<&Game> {
            self.games.get(&game_id)
        }
    };
    (ids) => {
        pub fn get_game_by_ids(&self, game_ids: Vec<u32>) -> Vec<&Game> {
            let mut games: Vec<&Game> = Vec::new();
            for game_id in game_ids {
                if let Some(game) = self.get_game_by_id(game_id) {
                    games.push(game);
                }
            }
            games.sort();
            games
        }
    };
    ($field:ident) => {
        paste! {
            pub fn [<get_game_by_ $field>](&self, field: &str) -> Vec<&Game> {
                match self.[<$field s>].get(field) {
                    Some(game_ids) => {
                        let mut games: Vec<&Game> = Vec::new();
                        for game_id in game_ids {
                            if let Some(game) = self.games.get(game_id) {
                                games.push(game)
                            }
                        }
                        games.sort();
                        games
                    }
                    None => vec![],
                }
            }
        }
    };
}
macro_rules! get_all {
    (games) => {
        pub fn get_all_games(&self) -> Vec<&Game> {
            let mut games: Vec<&Game> = self.games.values().collect();
            games.sort();
            games
        }
    };
    ($field:ident) => {
        paste! {
            pub fn [<get_all_ $field>](&self) -> Vec<&String> {
                let mut items: Vec<&String> = self.$field.keys().collect();
                items.sort();
                items
            }
        }
    };
}

impl GameDataBase {
    // Game queries
    get_all!(games);
    get_game_by!(id);
    get_game_by!(ids);
    get_game_by!(tag);
    get_game_by!(year);
    get_game_by!(engine);
    get_game_by!(runtime);
    get_game_by!(genre);
    get_game_by!(dev);
    get_game_by!(publi);

    // Other queries
    get_all!(tags);
    get_all!(engines);
    get_all!(runtimes);
    get_all!(genres);
    get_all!(years);
    get_all!(devs);
    get_all!(publis);
}
