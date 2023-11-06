#[cfg(test)]
mod tests {
    use std::{collections::HashMap, error::Error};

    #[test]
    fn create_remove_get() {
        let mut map = HashMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.remove("key2");

        assert_eq!(map.get("key1"), Some(&1));
        assert_eq!(map.get("key2"), None);
    }

    #[test]
    fn iterate_tuple() {
        let mut map = HashMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.insert("key3", 3);

        for (key, value) in map.iter() {
            println!("{key}, {value}");
        }
    }

    #[test]
    fn iterate_keys() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 1);
        map.insert("key2".to_string(), 2);
        map.insert("key3".to_string(), 3);

        for key in map.keys() {
            println!("{key}");
        }
    }

    #[test]
    fn iterate_values() {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("key1", "1");
        map.insert("key2", "2");
        map.insert("key3", "3");

        for key in map.values() {
            println!("{key}");
        }
    }

    #[test]
    fn contains_key() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "1".to_string());
        map.insert("key2".to_string(), "2".to_string());

        if map.contains_key("key1") {
            let k1 = map.get("key3").unwrap();
            println!("{:?}", k1);
        }

        // result
        let k1 = map.get("key3").ok_or_else(|| "не найден ключ key3".to_string());
        println!("{:?}", k1);
    }
}
