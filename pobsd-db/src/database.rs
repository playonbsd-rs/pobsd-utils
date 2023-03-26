//! Provide a representation of the PlayOnBSD database than can be
//! queried using a set of predefined methods.
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
