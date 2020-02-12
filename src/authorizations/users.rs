fn authorized_users() -> Vec<String> {
    dotenv::var("AUTHORIZED_USERS")
        .unwrap()
        .split(',')
        .map(|user| String::from(user))
        .collect()
}

pub fn is_authorized(name: String) -> bool {
    authorized_users().contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // The Authorized users environmental variable
    // is set for tests in the .env file

    #[test]
    fn list_authorized_users() {
        let result = authorized_users();
        assert!(
            result.contains(&String::from("luke")),
            "Result does not contain the expected name. Result was {:?}",
            result
        );

        assert!(
            result.contains(&String::from("leia")),
            "Result does not contain the expected name. Result was {:?}",
            result
        );
    }

    #[test]
    fn check_whether_user_is_authorized() {
        assert!(is_authorized(String::from("luke")));
        assert!(is_authorized(String::from("leia")));
        assert!(!is_authorized(String::from("chewie")));
    }
}
