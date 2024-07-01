use std::collections::HashMap;

pub struct Dict<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> Dict<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        Dict {
            map: HashMap::new(),
        }
    }

    pub fn create(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    pub fn add(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }

    pub fn replace(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key, value);
        }
    }

    pub fn fetch_value(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_random_key(&self) -> Option<&K> {
        self.map.keys().next()
    }

    pub fn delete(&mut self, key: &K) {
        self.map.remove(key);
    }

    pub fn release(&mut self) {
        self.map.clear();
    }
}