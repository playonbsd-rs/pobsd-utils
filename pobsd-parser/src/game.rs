use crate::store_links::StoreLinks;
use std::cmp::{Ordering, PartialOrd};
use std::fmt;

/// Represents a game from the database.
///
/// It also includes an additional [`Game::uid`] field
/// derived from the name of the game as well as the date to
/// which the game was added to the database. It therefore
/// provides an unique identifier under the assumption that no
/// game with the same name will be added the same dat into
/// the databas.
///
/// The name of some fields differs from the one used
/// in the database itself: Genre and Store are plural
/// since there can be more than one item for each
/// and Pub translate to publi since pub is a reserved
/// keyword in Rust.
///
/// All fields are optional strings or vectors of strings
/// except for the name of the game which is mandatory.
/// The parser does not try to be smart with dates and
/// just store them as string.
///
/// ### Display
/// The [`Game`] struct implement the [`core::fmt::Display`] trait
/// and will be displayed as it would appear in the
/// PlayOnBSD database.
///
/// ### PartialOrd
/// The [`Game`] struct implements the [`core::cmp::PartialOrd`] trait
/// and [`Game`] objects are ordered according to their name (without The or A).
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Game {
    /// An unique identifier generated from the name and added fields
    pub uid: u32,
    /// The name of the game.
    pub name: String,
    /// The cover of the game.
    pub cover: Option<String>,
    /// The engine used by the game.
    pub engine: Option<String>,
    /// Step(s) to setup the game.
    pub setup: Option<String>,
    /// The executable in the package.
    pub runtime: Option<String>,
    /// A vector with store urls.
    pub stores: Option<StoreLinks>,
    /// Hints (as the name imply).
    pub hints: Option<String>,
    /// A vector of genres associated with the game.
    pub genres: Option<Vec<String>>,
    /// A vector of tags associated with the game.
    pub tags: Option<Vec<String>>,
    /// Released year (can be text such as "early access".
    pub year: Option<String>,
    /// Developer.
    pub dev: Option<String>,
    /// Publisher.
    #[serde(rename = "pub")]
    pub publi: Option<String>,
    /// Version of the game.
    pub version: Option<String>,
    /// When tested on -current.
    pub status: Option<String>,
    /// When added
    pub added: Option<String>,
    /// When updated
    pub updated: Option<String>,
    /// The IGDB Id of the game
    pub igdb_id: Option<String>,
}

impl<'a> Game {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    fn get_ordering_name(&'a self) -> String {
        if let Some(name) = self.name.to_lowercase().strip_prefix("the ") {
            return name.to_string();
        }
        if let Some(name) = self.name.to_lowercase().strip_prefix("a ") {
            return name.to_string();
        }
        self.name.to_lowercase()
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Game) -> Option<Ordering> {
        self.get_ordering_name()
            .partial_cmp(&other.get_ordering_name())
    }
    fn lt(&self, other: &Game) -> bool {
        self.get_ordering_name().lt(&other.get_ordering_name())
    }
    fn le(&self, other: &Game) -> bool {
        self.get_ordering_name().le(&other.get_ordering_name())
    }
    fn gt(&self, other: &Game) -> bool {
        self.get_ordering_name().gt(&other.get_ordering_name())
    }
    fn ge(&self, other: &Game) -> bool {
        self.get_ordering_name().ge(&other.get_ordering_name())
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Game) -> Ordering {
        self.get_ordering_name().cmp(&other.get_ordering_name())
    }
}

/// Display the game as it would appears in the database.
/// See <https://github.com/playonbsd/OpenBSD-Games-Database>
/// for details.
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let game = format!("Game\t{}", self.name);
        let cover = match &self.cover {
            Some(cover) => format!("Cover\t{}", cover),
            None => "Cover".to_string(),
        };
        let engine = match &self.engine {
            Some(engine) => format!("Engine\t{}", engine),
            None => "Engine".to_string(),
        };
        let setup = match &self.setup {
            Some(setup) => format!("Setup\t{}", setup),
            None => "Setup".to_string(),
        };
        let runtime = match &self.runtime {
            Some(runtime) => format!("Runtime\t{}", runtime),
            None => "Runtime".to_string(),
        };
        let stores = match &self.stores {
            Some(stores) => format!(
                "Store\t{}",
                stores
                    .inner_ref()
                    .into_iter()
                    .map(|a| a.url.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            None => "Store".to_string(),
        };
        let hints = match &self.hints {
            Some(hints) => format!("Hints\t{}", hints),
            None => "Hints".to_string(),
        };
        let genres = match &self.genres {
            Some(genres) => format!("Genre\t{}", genres.join(", ")),
            None => "Genre".to_string(),
        };
        let tags = match &self.tags {
            Some(tags) => format!("Tags\t{}", tags.join(", ")),
            None => "Tags".to_string(),
        };
        let year = match &self.year {
            Some(year) => format!("Year\t{}", year),
            None => "Year".to_string(),
        };
        let dev = match &self.dev {
            Some(dev) => format!("Dev\t{}", dev),
            None => "Dev".to_string(),
        };
        let publi = match &self.publi {
            Some(publi) => format!("Pub\t{}", publi),
            None => "Pub".to_string(),
        };
        let version = match &self.version {
            Some(version) => format!("Version\t{}", version),
            None => "Version".to_string(),
        };
        let status = match &self.status {
            Some(status) => format!("Status\t{}", status),
            None => "Status".to_string(),
        };
        let added = match &self.added {
            Some(added) => format!("Added\t{}", added),
            None => "Added".to_string(),
        };
        let updated = match &self.updated {
            Some(updated) => format!("Updated\t{}", updated),
            None => "Updated".to_string(),
        };
        let igdb_id = match &self.igdb_id {
            Some(runtime) => format!("IgdbId\t{}", runtime),
            None => "IgdbId".to_string(),
        };
        write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            game,
            cover,
            engine,
            setup,
            runtime,
            stores,
            hints,
            genres,
            tags,
            year,
            dev,
            publi,
            version,
            status,
            added,
            updated,
            igdb_id,
        )
    }
}

/* ------------------------- TESTS --------------------------*/

#[cfg(test)]
mod game_tests {
    use crate::store_links::StoreLink;

    use super::*;
    fn create_game() -> Game {
        let mut game = Game::default();
        let tags: Vec<String> = vec!["tag1".to_string(), "tag2".to_string()];
        let genres: Vec<String> = vec!["genre1".to_string(), "genre2".to_string()];
        let stores: Vec<String> = vec!["store1".to_string(), "store2".to_string()];
        let store_links: Vec<StoreLink> = stores.into_iter().map(|a| StoreLink::from(&a)).collect();
        let stores = StoreLinks(store_links);
        game.uid = 1221;
        game.name = "game name".to_string();
        game.cover = Some("cover.jpg".to_string());
        game.engine = Some("game engine".to_string());
        game.setup = Some("game setup".to_string());
        game.runtime = Some("game runtime".to_string());
        game.stores = Some(stores);
        game.hints = Some("game hints".to_string());
        game.genres = Some(genres);
        game.tags = Some(tags);
        game.year = Some("1980".to_string());
        game.dev = Some("game dev".to_string());
        game.publi = Some("game publi".to_string());
        game.version = Some("game version".to_string());
        game.status = Some("game status".to_string());
        game.added = Some("2012-12-03".to_string());
        game.updated = Some("2014-12-03".to_string());
        game
    }
    #[test]
    fn test_default_equivalent_to_new() {
        let game = Game::default();
        let game_bis = Game::new();
        assert!(game == game_bis);
    }
    #[test]
    fn test_get_ordering_name_with_a() {
        let mut game = create_game();
        game.name = "A champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
        game.name = "a champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
    }
    #[test]
    fn test_get_ordering_name_with_the() {
        let mut game = create_game();
        game.name = "The champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
        game.name = "the champion".into();
        assert_eq!(game.get_ordering_name(), "champion");
    }
    #[test]
    fn test_ordering() {
        let mut game1 = create_game();
        let mut game2 = create_game();
        game1.name = "Abc".into();
        game2.name = "Def".into();
        assert!(game2.gt(&game1));
        assert!(game2.ge(&game1));
        assert!(game1.le(&game2));
        assert!(game1.lt(&game2));
        game1.name = "The Abc".into();
        game2.name = "def".into();
        assert!(game2.gt(&game1));
        assert!(game2.ge(&game1));
        assert!(game1.le(&game2));
        assert!(game1.lt(&game2));
        game1.name = "The Abc".into();
        game2.name = "A def".into();
        assert!(game2.gt(&game1));
        assert!(game2.ge(&game1));
        assert!(game1.le(&game2));
        assert!(game1.lt(&game2));
    }
    #[test]
    fn test_display() {
        let game_str = "Game\tAaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
Cover\tAaaaaA_for_the_Awesome_Cover.jpg
Engine
Setup
Runtime\tHumblePlay
Store\thttps://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome
Hints\tDemo on HumbleBundle store page
Genre
Tags
Year\t2011
Dev
Pub
Version
Status
Added
Updated
IgdbId";
        let game = Game {
            uid: 12,
            name: "AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome".to_string(),
            cover: Some("AaaaaA_for_the_Awesome_Cover.jpg".to_string()),
            engine: None,
            setup: None,
            runtime: Some("HumblePlay".to_string()),
            stores: Some(StoreLinks(vec![StoreLink::from(
                "https://www.humblebundle.com/store/aaaaaaaaaaaaaaaaaaaaaaaaa-for-the-awesome",
            )])),
            hints: Some("Demo on HumbleBundle store page".to_string()),
            genres: None,
            tags: None,
            year: Some("2011".to_string()),
            dev: None,
            publi: None,
            version: None,
            status: None,
            added: None,
            updated: None,
            igdb_id: None,
        };
        assert_eq!(format!("{}", game), game_str);
    }
}
