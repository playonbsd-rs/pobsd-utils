use crate::query_result::QueryResult;
use paste::paste;
use pobsd_parser::Game;

use crate::database::GameDataBase;

macro_rules! get_game_by {
    (id) => {
        /// Return the game with a given id
        pub fn get_game_by_id(&self, game_id: u32) -> Option<&Game> {
            self.games.get(&game_id)
        }
    };
    (ids) => {
        /// Return the game with the given ids
        pub fn get_game_by_ids(&self, game_ids: Vec<u32>) -> QueryResult<&Game> {
            let mut games: Vec<&Game> = Vec::new();
            for game_id in game_ids {
                if let Some(game) = self.get_game_by_id(game_id) {
                    games.push(game);
                }
            }
            games.sort();
            QueryResult { items: games }
        }
    };
    (name) => {
        /// Return the game with the given name (case sensitive)
        pub fn get_game_by_name(&self, name: &str) -> Option<&Game> {
            for game in self.games.values() {
                if game.name.eq(name) {
                    return Some(game);
                }
            }
            None
        }
    };
    ($field:ident) => {
        paste! {
            /// Return the games having the chosen field equal to the given value
            pub fn [<get_game_by_ $field>](&self, field: &str) -> QueryResult<&Game> {
                match self.[<$field s>].get(field) {
                    Some(game_ids) => {
                        let mut games: Vec<&Game> = Vec::new();
                        for game_id in game_ids {
                            if let Some(game) = self.games.get(game_id) {
                                games.push(game)
                            }
                        }
                        games.sort();
                        QueryResult{ items: games}
                    }
                    None => QueryResult{ items : vec![]},
                }
            }
        }
    };
}

macro_rules! search_game_by {
    (name) => {
        /// Return the games having the name containing the given value (not case sensitive)
        pub fn search_game_by_name(&self, name: &str) -> QueryResult<&Game> {
            let mut games: Vec<&Game> = Vec::new();
            for game in self.games.values() {
                if game.name.to_lowercase().contains(&name.to_lowercase()) {
                    games.push(game)
                }
            }
            QueryResult { items: games }
        }
    };
    ($field:ident) => {
        paste! {
            /// Return the games having the given field containing the given value (not case sensitive)
            pub fn [<search_game_by_ $field>](&self, name: &str) -> QueryResult<&Game> {
                let mut games: Vec<&Game> = Vec::new();
                for game in self.games.values() {
                    if let Some(value) = &game.$field {
                        if value.to_lowercase().contains(&name.to_lowercase()) {
                            games.push(game);
                        }
                    }
                }
                QueryResult { items: games }
            }
        }
    };
    (array $field:ident) => {
        paste! {
            /// Return the games having the given field containing the given value (not case sensitive)
            pub fn [<search_game_by_ $field>](&self, name: &str) -> QueryResult<&Game> {
                let mut games: Vec<&Game> = Vec::new();
                for game in self.games.values() {
                    if let Some(value) = &game.$field {
                        if value.join(" ").to_lowercase().contains(&name.to_lowercase()) {
                            games.push(game);
                        }
                    }
                }
                QueryResult { items: games }
            }
        }
    };
}

macro_rules! get_all {
    (games) => {
        /// Return all the games of the database
        pub fn get_all_games(&self) -> QueryResult<&Game> {
            let mut games: Vec<&Game> = self.games.values().collect();
            games.sort();
            QueryResult { items: games }
        }
    };
    ($field:ident) => {
        paste! {
            /// Return all the chosen items of the database
            pub fn [<get_all_ $field>](&self) -> QueryResult<&String> {
                let mut items: Vec<&String> = self.$field.keys().collect();
                items.sort();
                QueryResult{items}
            }
        }
    };
}

impl GameDataBase {
    // Game queries
    get_all!(games);
    get_game_by!(id);
    get_game_by!(ids);
    get_game_by!(name);
    get_game_by!(tag);
    get_game_by!(year);
    get_game_by!(engine);
    get_game_by!(runtime);
    get_game_by!(genre);
    get_game_by!(dev);
    get_game_by!(publi);

    search_game_by!(name);
    search_game_by!(year);
    search_game_by!(engine);
    search_game_by!(runtime);
    search_game_by!(dev);
    search_game_by!(publi);
    search_game_by!(array genres);
    search_game_by!(array tags);
    // Other queries
    get_all!(tags);
    get_all!(engines);
    get_all!(runtimes);
    get_all!(genres);
    get_all!(years);
    get_all!(devs);
    get_all!(publis);
}
