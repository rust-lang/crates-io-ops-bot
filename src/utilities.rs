use std::collections::HashSet;

pub fn parse_config_value_set(config_value: String) -> HashSet<String> {
    config_value.split(',').map(String::from).collect()
}

pub fn parse_config_value_string(config_value: HashSet<String>) -> String {
    let non_empty: Vec<String> = config_value.into_iter().filter(|s| !s.is_empty()).collect();
    non_empty.join(",")
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

    #[test]
    fn create_authorized_users_string() {
        let mut users_hash_set = HashSet::new();
        users_hash_set.insert("123".to_string());
        users_hash_set.insert("456".to_string());
        users_hash_set.insert("789".to_string());

        let users_string = parse_config_value_string(users_hash_set);
        assert!(users_string.contains("123"));
        assert!(users_string.contains("456"));
        assert!(users_string.contains("789"));
    }
}
