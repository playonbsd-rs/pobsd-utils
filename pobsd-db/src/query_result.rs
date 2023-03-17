use paste::paste;
use pobsd_parser::Game;

macro_rules! get_game_by {
    ($field:ident) => {
        paste! {
            pub fn [<get_game_by_ $field>](self, field: &str) -> QueryResult<T> {
                let mut items: Vec<T> = self
                    .items
                    .clone()
                    .into_iter()
                    .filter(|a| a.[<get_ $field>]().eq(&Some(field.to_string())))
                    .collect();
                items.sort();
                QueryResult{items}
            }
        }
    };
    (array $field:ident) => {
        paste! {
            pub fn [<get_game_by_ $field>](self, field: &str) -> QueryResult<T> {
                let mut items: Vec<T> = self
                    .items
                    .clone()
                    .into_iter()
                    .filter(|a| match a.[<get_ $field>]() {
                        Some(items) => items.contains(&field.to_string()),
                        None => false,
                    })
                    .collect();
                items.sort();
                QueryResult{items}
            }
        }
    };
}

pub trait Item {
    fn get_name(&self) -> &str;
}

impl Item for &Game {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Item for &String {
    fn get_name(&self) -> &str {
        &self
    }
}

pub trait GameItem: Item {
    fn get_uid(&self) -> u32;
    fn get_runtime(&self) -> &Option<String>;
    fn get_year(&self) -> &Option<String>;
    fn get_dev(&self) -> &Option<String>;
    fn get_publi(&self) -> &Option<String>;
    fn get_engine(&self) -> &Option<String>;
    fn get_genres(&self) -> &Option<Vec<String>>;
    fn get_tags(&self) -> &Option<Vec<String>>;
}

impl GameItem for &Game {
    fn get_uid(&self) -> u32 {
        self.uid
    }
    fn get_runtime(&self) -> &Option<String> {
        &self.runtime
    }
    fn get_year(&self) -> &Option<String> {
        &self.year
    }
    fn get_dev(&self) -> &Option<String> {
        &self.dev
    }
    fn get_publi(&self) -> &Option<String> {
        &self.publi
    }
    fn get_genres(&self) -> &Option<Vec<String>> {
        &self.genres
    }
    fn get_tags(&self) -> &Option<Vec<String>> {
        &self.genres
    }
    fn get_engine(&self) -> &Option<String> {
        &self.engine
    }
}

pub struct QueryResult<T: Item + Clone> {
    pub items: Vec<T>,
}

impl<T: Item + Clone> QueryResult<T> {
    pub fn get_by_name(&self, name: &str) -> Option<T> {
        let mut items: Vec<&T> = self
            .items
            .iter()
            .filter(|a| a.get_name().eq(name))
            .collect();
        items.pop().cloned()
    }
    pub fn search_by_name(self, name: &str) -> QueryResult<T> {
        let items: Vec<T> = self
            .items
            .clone()
            .into_iter()
            .filter(|a| a.get_name().contains(name))
            .collect();
        QueryResult { items }
    }
}

impl<T: GameItem + Clone + Ord> QueryResult<T> {
    get_game_by!(runtime);
    get_game_by!(year);
    get_game_by!(dev);
    get_game_by!(publi);
    get_game_by!(engine);
    get_game_by!(array genres);
    get_game_by!(array tags);
}
