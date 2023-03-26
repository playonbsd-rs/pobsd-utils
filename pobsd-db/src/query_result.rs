//! Provides a representation of the query result returned when
//! interogating the database. Query results are themselves queriable
//! and return another query result.
use crate::Item;
use paste::paste;
use pobsd_parser::Game;

macro_rules! get_game_by {
    ($field:ident) => {
        paste! {
            pub fn [<get_game_by_ $field>](self, field: &str) -> QueryResult<&'a Game> {
                let mut items: Vec<&Game> = self
                    .items
                    .clone()
                    .into_iter()
                    .filter(|a| a.$field.eq(&Some(field.to_string())))
                    .collect();
                items.sort();
                QueryResult{items}
            }
        }
    };
    (array $field:ident) => {
        paste! {
            pub fn [<get_game_by_ $field>](self, field: &str) -> QueryResult<&'a Game> {
                let mut items: Vec<&Game> = self
                    .items
                    .clone()
                    .into_iter()
                    .filter(|a| match &a.$field {
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

pub struct QueryResult<T> {
    pub items: Vec<T>,
}

impl QueryResult<Item> {
    pub fn get_item_by_name(&self, name: &str) -> Option<Item> {
        let mut items: Vec<&Item> = self.items.iter().filter(|a| a.eq(&name)).collect();
        items.pop().cloned()
    }
    pub fn search_item_by_name(self, name: &str) -> QueryResult<Item> {
        let items: Vec<Item> = self
            .items
            .into_iter()
            .filter(|a| a.contains(name))
            .collect();
        QueryResult { items }
    }
}

impl<'a> QueryResult<&'a Game> {
    pub fn get_game_by_name(self, name: &str) -> Option<&'a Game> {
        let mut items: Vec<&Game> = self.items.into_iter().filter(|a| a.name.eq(name)).collect();
        items.pop()
    }
    pub fn search_game_by_name(self, name: &str) -> QueryResult<&'a Game> {
        let items: Vec<&Game> = self
            .items
            .into_iter()
            .filter(|a| a.name.contains(name))
            .collect();
        QueryResult { items }
    }
    get_game_by!(runtime);
    get_game_by!(year);
    get_game_by!(dev);
    get_game_by!(publi);
    get_game_by!(engine);
    get_game_by!(array genres);
    get_game_by!(array tags);
}
