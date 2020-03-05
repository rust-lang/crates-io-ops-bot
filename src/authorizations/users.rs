use crate::config::Config;

fn authorized_users(config: &Config) -> Vec<String> {
    let split_string = config.authorized_users.split(',');
    let auth_users: Vec<String> = split_string.map(String::from).collect();

    auth_users
}

pub fn is_authorized(id: &str, config: &Config) -> bool {
    authorized_users(config).iter().any(|i| i == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn set_discord_token() {
        env::set_var("DISCORD_TOKEN", "abc123");
    }

    fn set_heroku_api_key() {
        env::set_var("HEROKU_API", "abc123");
    }

    fn set_authorized_users() {
        env::set_var("AUTHORIZED_USERS", "123,456");
    }

    #[test]
    fn list_authorized_users() {
        set_discord_token();
        set_heroku_api_key();
        set_authorized_users();

        let result = authorized_users(&Config::default());
        assert!(
            result.contains(&String::from("123")),
            "Result does not contain the expected name. Result was {:?}",
            result
        );

        assert!(
            result.contains(&String::from("456")),
            "Result does not contain the expected name. Result was {:?}",
            result
        );
    }

    #[test]
    fn check_whether_user_is_authorized() {
        let config = Config::default();
        set_authorized_users();

        assert!(is_authorized("123", &config));
        assert!(is_authorized("456", &config));
        assert!(!is_authorized("789", &config));
    }
}
