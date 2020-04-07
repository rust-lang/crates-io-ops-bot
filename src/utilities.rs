use std::collections::HashSet;

pub fn parse_config_value_set(config_value: String) -> HashSet<String> {
    let mut value_set = HashSet::new();

    let split_string = config_value.split(',');

    for string in split_string {
        value_set.insert(String::from(string));
    }

    value_set
}

pub fn parse_config_value_string(config_value: HashSet<String>) -> String {
    String::from("wheee");
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
        let users_hash_set = HashSet::new();
        users_hash_set.insert("123".to_string());
        users_hash_set.insert("456".to_string());
        users_hash_set.insert("789".to_string());

        let users_string = parse_config_value_string(users_hash_set);
        assert_eq!(users_string, "123,456,789");
    }
}
