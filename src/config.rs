use serenity::prelude::TypeMapKey;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_token: String,
    pub heroku_api_key: String,
    pub build_check_interval: u64,
    pub build_message_display_interval: u64,
    pub github_org: String,
    pub github_repo: String,
    pub github_token: String,
}

impl Config {
    pub fn new(
        discord_token: String,
        heroku_api_key: String,
        build_check_interval: String,
        build_message_display_interval: String,
        github_org: String,
        github_repo: String,
        github_token: String,
    ) -> Config {
        Config {
            discord_token,
            heroku_api_key,
            build_check_interval: build_check_interval.parse::<u64>().unwrap(),
            build_message_display_interval: build_message_display_interval.parse::<u64>().unwrap(),
            github_org,
            github_repo,
            github_token,
        }
    }
}

impl TypeMapKey for Config {
    type Value = Arc<Config>;
}
