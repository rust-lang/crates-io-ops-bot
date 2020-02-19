fn authorized_users() -> Vec<String> {
    dotenv::var("AUTHORIZED_USERS")
        .unwrap()
        .split(',')
        .map(String::from)
        .collect()
}

pub fn is_authorized(id: &str) -> bool {
   authorized_users().contains(&String::from(id))
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    #[test]
    fn list_authorized_users() {
        let test_auth_users = "123,456";
        env::set_var("AUTHORIZED_USERS", test_auth_users);

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
        assert!(is_authorized("123"));
        assert!(is_authorized("456"));
        assert!(!is_authorized("789"));
    }
}
