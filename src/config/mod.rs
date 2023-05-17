use serde::{Deserialize, Serialize};

pub type RssUrl = String;
pub type Frequency = i64;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigFeed {
    url: RssUrl,
    name: String,
    number_of_posts: i64,
    only_new_posts: bool,
    update_frequency: Frequency,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    welcome_title_format: String,
    update_frequency: Frequency,
}

impl Config {
    pub fn new() -> Self {
        Config {
            update_frequency: (60 * 4), // Update every 4 hours
            welcome_title_format: "Welcome %name.".to_string(),
        }
    }

    pub fn welcome_format(&self) -> &str {
        return self.welcome_title_format.as_str();
    }
}