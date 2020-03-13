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

fn parse_config_value_set(config_value: String) -> HashSet<String> {
    let mut value_set = HashSet::new();

    let split_string = config_value.split(',');

    for string in split_string {
        value_set.insert(String::from(string));
    }

    value_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_authorized_users_hashset() {
        let test_string = String::from("123,456,789");
        let users_set = parse_config_value_set(test_string);

        assert!(users_set.contains("123"));
        assert!(users_set.contains("456"));
        assert!(users_set.contains("789"));
    }
}
