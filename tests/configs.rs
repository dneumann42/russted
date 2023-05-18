use std::{fs::remove_file, path::Path};

use russted::config::{
    io::ConfigReader,
    io::ConfigWriter,
    toml_io::TomlIO,
    types::{Config, ConfigFeed},
};

#[test]
fn that_we_can_construct_the_config() {
    assert_eq!(Config::new().welcome_format(), "Welcome %name.");
}

// I would like to use an in memory file system to do this, but I haven't
// found a library that I like yet.

#[test]
fn that_we_can_save_and_load_config() {
    let mut cfg = Config::new();
    cfg.add_feed(ConfigFeed::new(
        "https://indiegamesplus.com/feed",
        "indie games plus",
    ));
    cfg.add_feed(ConfigFeed::new(
        "https://store.steampowered.com/feeds/news.xml",
        "Steam RSS News Feed",
    ));
    assert!(TomlIO::write(&cfg, "test.toml").is_none());
    assert_eq!(TomlIO::read("test.toml").unwrap(), cfg);
    remove_file("test.toml").unwrap();
}
