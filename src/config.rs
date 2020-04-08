use crate::utilities::*;
use serenity::prelude::TypeMapKey;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Config {
    pub discord_token: String,
    pub authorized_users: HashSet<String>,
    pub heroku_api_key: String,
}

impl Config {
    pub fn new(discord_token: String, authorized_users: String, heroku_api_key: String) -> Config {
        Config {
            discord_token,
            authorized_users: parse_config_value_set(authorized_users),
            heroku_api_key,
        }
    }
}

impl TypeMapKey for Config {
    type Value = Arc<Config>;
}
