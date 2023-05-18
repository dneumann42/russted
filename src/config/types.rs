use serde::{Deserialize, Serialize};

const DEFAULT_LINUX_CONFIG_PATH: &'static str = ".local/share/russted.toml";

pub type RssUrl = String;
pub type Frequency = i64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ConfigFeed {
    url: RssUrl,
    name: String,
    number_of_posts: i64,
    only_new_posts: bool,
    update_frequency: Frequency,
}

impl ConfigFeed {
    pub fn new<A: Into<String>, B: Into<String>>(url: A, name: B) -> ConfigFeed {
        Self {
            url: url.into(),
            name: name.into(),
            number_of_posts: -1,
            update_frequency: -1,
            only_new_posts: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Config {
    cache_directory: String,
    welcome_title_format: String,
    update_frequency: Frequency,
    feeds: Vec<ConfigFeed>,
}

impl Config {
    pub fn cache_directory() -> String {
        dirs::home_dir()
            .map(|p| p.join(DEFAULT_LINUX_CONFIG_PATH.to_string()))
            .map_or_else(
                || "".to_owned(),
                |p| p.as_path().to_str().unwrap_or_default().to_owned(),
            )
    }

    pub fn new() -> Self {
        Config {
            update_frequency: (60 * 4), // Update every 4 hours
            welcome_title_format: "Welcome %name.".to_string(),
            cache_directory: Config::cache_directory(),
            feeds: vec![],
        }
    }

    pub fn welcome_format(&self) -> &str {
        return self.welcome_title_format.as_str();
    }

    pub fn add_feed(&mut self, feed: ConfigFeed) {
        self.feeds.push(feed)
    }
}
