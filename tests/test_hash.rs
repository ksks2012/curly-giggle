use curly_giggle::collection::hash::Dict;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut dict = Dict::new();
        dict.create("key1", "value1");
        assert_eq!(dict.fetch_value(&"key1"), Some(&"value1"));
    }

    #[test]
    fn test_add() {
        let mut dict = Dict::new();
        dict.add("key1", "value1");
        assert_eq!(dict.fetch_value(&"key1"), Some(&"value1"));
    }

    #[test]
    fn test_replace_existing_key() {
        let mut dict = Dict::new();
        dict.create("key1", "value1");
        dict.replace("key1", "new_value");
        assert_eq!(dict.fetch_value(&"key1"), Some(&"new_value"));
    }

    #[test]
    fn test_replace_non_existing_key() {
        let mut dict = Dict::new();
        dict.replace("key1", "value1");
        assert_eq!(dict.fetch_value(&"key1"), None);
    }

    #[test]
    fn test_fetch_value_existing_key() {
        let mut dict = Dict::new();
        dict.create("key1", "value1");
        assert_eq!(dict.fetch_value(&"key1"), Some(&"value1"));
    }

    #[test]
    fn test_fetch_value_non_existing_key() {
        let dict: Dict<&str, &str> = Dict::new();
        assert_eq!(dict.fetch_value(&"key1"), None);
    }

    #[test]
    fn test_get_random_key() {
        let mut dict = Dict::new();
        dict.create("key1", "value1");
        assert_eq!(dict.get_random_key(), Some(&"key1"));
    }

    #[test]
    fn test_delete_existing_key() {
        let mut dict = Dict::new();
        dict.create("key1", "value1");
        dict.delete(&"key1");
        assert_eq!(dict.fetch_value(&"key1"), None);
    }

    #[test]
    fn test_delete_non_existing_key() {
        let mut dict: Dict<&str, &str> = Dict::new();
        dict.delete(&"key1");
        assert_eq!(dict.fetch_value(&"key1"), None);
    }

    #[test]
    fn test_release() {
        let mut dict = Dict::new();
        dict.create("key1", "value1");
        dict.release();
        assert_eq!(dict.fetch_value(&"key1"), None);
    }
}