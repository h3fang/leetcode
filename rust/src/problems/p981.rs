use std::collections::{BTreeMap, HashMap};

#[derive(Default)]
pub struct TimeMap {
    map: HashMap<String, BTreeMap<i32, String>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().insert(timestamp, value);
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        self.map
            .get(&key)
            .map(|m| {
                m.range(..=timestamp)
                    .next_back()
                    .map(|e| e.1.to_string())
                    .unwrap_or_default()
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut tm = TimeMap::new();
        tm.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!("bar", tm.get("foo".to_string(), 1));
        assert_eq!("bar", tm.get("foo".to_string(), 3));
        tm.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!("bar2", tm.get("foo".to_string(), 4));
        assert_eq!("bar2", tm.get("foo".to_string(), 5));
    }
}
