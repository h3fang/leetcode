use rand::prelude::*;
use std::collections::{HashMap, hash_map::Entry};

pub struct RandomizedSet {
    index: HashMap<i32, usize>,
    arr: Vec<i32>,
}

impl RandomizedSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            arr: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if let Entry::Vacant(e) = self.index.entry(val) {
            e.insert(self.arr.len());
            self.arr.push(val);
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some((_, idx)) = self.index.remove_entry(&val) {
            if idx < self.arr.len() - 1 {
                let last = self.arr.last().unwrap();
                self.index.insert(*last, idx);
                self.arr[idx] = *last;
            }
            self.arr.pop();
            true
        } else {
            false
        }
    }

    pub fn get_random(&self) -> i32 {
        let i = rand::rng().random_range(0..self.arr.len());
        self.arr[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut s = RandomizedSet::new();
        assert!(s.insert(1));
        assert!(!s.remove(2));
        assert!(s.insert(2));
        assert!([1, 2].contains(&s.get_random()));
        assert!(s.remove(1));
        assert!(!s.insert(2));
        assert_eq!(2, s.get_random());
    }

    #[test]
    fn case2() {
        let mut s = RandomizedSet::new();
        assert!(s.insert(1));
        assert!(s.remove(1));
        assert!(s.insert(-1));
        assert!(!s.remove(1));
        assert_eq!(-1, s.get_random());
        assert_eq!(-1, s.get_random());
        assert_eq!(-1, s.get_random());
        assert_eq!(-1, s.get_random());
        assert_eq!(-1, s.get_random());
    }
}
