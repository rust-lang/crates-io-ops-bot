use crate::config::config::Config;

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

    fn test_config() -> Config {
        Config::new(
            String::from("123"),
            String::from("123,456"),
            String::from("456"),
        )
    }

    #[test]
    fn list_authorized_users() {
        let config = test_config();
        let result = authorized_users(&config);
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
        let config = test_config();

        assert!(is_authorized("123", &config));
        assert!(is_authorized("456", &config));
        assert!(!is_authorized("789", &config));
    }
}
