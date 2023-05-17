use russted::config::Config;

#[test]
fn that_we_can_construct_the_config() {
    assert_eq!(Config::new().welcome_format(), "Welcome %name.");
}
