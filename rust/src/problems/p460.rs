use std::collections::{BTreeSet, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Entry {
    freq: i32,
    time: i32,
    key: i32,
    value: i32,
}

pub struct LFUCache {
    capacity: usize,
    time: i32,
    cache: HashMap<i32, Entry>,
    set: BTreeSet<Entry>,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            capacity: capacity as usize,
            time: 0,
            cache: Default::default(),
            set: Default::default(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(e) = self.cache.get_mut(&key) {
            self.set.remove(e);
            e.freq += 1;
            self.time += 1;
            e.time = self.time;
            self.set.insert(*e);
            e.value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        self.time += 1;
        if let Some(e) = self.cache.get_mut(&key) {
            self.set.remove(e);
            e.freq += 1;
            e.value = value;
            e.time = self.time;
            self.set.insert(*e);
        } else {
            if self.cache.len() == self.capacity {
                let e = *self.set.iter().next().unwrap();
                self.set.remove(&e);
                self.cache.remove(&e.key);
            }
            let entry = Entry {
                freq: 0,
                time: self.time,
                key,
                value,
            };
            self.set.insert(entry);
            self.cache.insert(key, entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut lfu = LFUCache::new(2);
        lfu.put(1, 1);
        lfu.put(2, 2);
        assert_eq!(1, lfu.get(1));
        lfu.put(3, 3);
        assert_eq!(-1, lfu.get(2));
        assert_eq!(3, lfu.get(3));
        lfu.put(4, 4);
        assert_eq!(-1, lfu.get(1));
        assert_eq!(3, lfu.get(3));
        assert_eq!(4, lfu.get(4));
    }
}
