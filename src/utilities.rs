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
    fn create_ips_hashset() {
        let test_string = String::from("123.0.0.1,456.0.0.1,789.0.0.1");
        let ips_set = parse_config_value_set(test_string);

        assert!(ips_set.contains("123.0.0.1"));
        assert!(ips_set.contains("456.0.0.1"));
        assert!(ips_set.contains("789.0.0.1"));
    }

    #[test]
    fn create_authorized_ips_string() {
        let mut ips_hash_set = HashSet::new();
        ips_hash_set.insert("123.0.0.1".to_string());
        ips_hash_set.insert("456.0.0.1".to_string());
        ips_hash_set.insert("789.0.0.1".to_string());

        let ips_string = parse_config_value_string(ips_hash_set);
        assert!(ips_string.contains("123.0.0.1"));
        assert!(ips_string.contains("456.0.0.1"));
        assert!(ips_string.contains("789.0.0.1"));
    }
}
