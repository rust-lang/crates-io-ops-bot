use crate::config::Config;

pub fn is_authorized(id: &str, config: &Config) -> bool {
    config.authorized_users.contains(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::new(
            String::from("123"),
            String::from("123,456"),
            String::from("456abc"),
            String::from("5"),
            String::from("github_org"),
            String::from("github_repo"),
            String::from("789xyz"),
        )
    }

    #[test]
    fn check_whether_user_is_authorized() {
        let config = test_config();

        assert!(is_authorized("123", &config.clone()));
        assert!(is_authorized("456", &config.clone()));
        assert!(!is_authorized("789", &config.clone()));
    }
}
