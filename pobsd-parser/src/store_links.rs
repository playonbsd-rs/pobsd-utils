use regex::Regex;

#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum Store {
    Steam,
    Gog,
    #[default]
    Unknown,
}
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct StoreLink {
    pub store: Store,
    pub url: String,
}

impl StoreLink {
    pub fn from(url: &str) -> Self {
        if url.contains("steampowered") {
            Self {
                store: Store::Steam,
                url: url.to_string(),
            }
        } else if url.contains("gog.com") {
            Self {
                store: Store::Gog,
                url: url.to_string(),
            }
        } else {
            Self {
                store: Store::Unknown,
                url: url.to_string(),
            }
        }
    }
    pub fn get_id(&self) -> Option<usize> {
        let re = Regex::new(r"https://store.steampowered.com/app/(\d+)(/?.+)?").unwrap();
        match &self.store {
            Store::Steam => {
                let cap = re.captures(&self.url).unwrap();
                if let Some(cap) = cap.get(1) {
                    return cap.as_str().parse::<usize>().ok();
                };
                None
            }
            _ => None,
        }
    }
}

#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct StoreLinks(pub Vec<StoreLink>);

impl StoreLinks {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, store: StoreLink) {
        self.0.push(store)
    }
    pub fn inner_ref(&self) -> &Vec<StoreLink> {
        &self.0
    }
    pub fn inner_mut_ref(&mut self) -> &mut Vec<StoreLink> {
        &mut self.0
    }
    pub fn into_inner(self) -> Vec<StoreLink> {
        self.0
    }
}

#[cfg(test)]
mod store_link_tests {
    use super::*;
    #[test]
    fn test_get_id_steam() {
        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910/LoupLaine/".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));

        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));

        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910/".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));

        let store = StoreLink {
            store: Store::Steam,
            url: "https://store.steampowered.com/app/1878910/LoupLaine".to_string(),
        };
        assert_eq!(store.get_id(), Some(1878910));
    }
    #[test]
    fn test_get_id_gog() {
        let store = StoreLink {
            store: Store::Gog,
            url: "https://store.steampowered.com/app/1878910/LoupLaine/".to_string(),
        };
        assert_eq!(store.get_id(), None);
    }
}
