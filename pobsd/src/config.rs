use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum IgdbStatus {
    #[default]
    Disabled,
    Enabled,
}
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub igdb_cid: Option<String>,
    pub igdb_secret: Option<String>,
    pub igdb_enable: Option<IgdbStatus>,
}
