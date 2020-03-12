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
            authorized_users: authorized_users_set(authorized_users),
            heroku_api_key,
        }
    }
}

impl TypeMapKey for Config {
    type Value = Arc<Config>;
}

fn authorized_users_set(users: String) -> HashSet<String> {
    let mut users_set = HashSet::new();

    let split_string = users.split(',');

    for string in split_string {
        users_set.insert(String::from(string));
    }

    users_set
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_authorized_users_hashset() {
        let test_string = String::from("123,456,789");
        let users_set = authorized_users_set(test_string);

        assert!(users_set.contains("123"));
        assert!(users_set.contains("456"));
        assert!(users_set.contains("789"));
    }
}
