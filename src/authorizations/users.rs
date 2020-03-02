use crate::config::Config;

fn authorized_users() -> Vec<String> {
    let bot_config = Config::default();

    let split_string = bot_config.authorized_users.split(',');
    let auth_users: Vec<String> = split_string.map(String::from).collect();

    auth_users
}

pub fn is_authorized(id: &str) -> bool {
    authorized_users().iter().any(|i| i == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn set_authorized_users() {
        env::set_var("AUTHORIZED_USERS", "123,456");
    }

    #[test]
    fn list_authorized_users() {
        set_authorized_users();

        let result = authorized_users();
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
        set_authorized_users();

        assert!(is_authorized("123"));
        assert!(is_authorized("456"));
        assert!(!is_authorized("789"));
    }
}
