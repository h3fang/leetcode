const BASE: usize = 10009;

pub struct MyHashSet {
    s: Vec<Vec<i32>>,
}

impl MyHashSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            s: vec![vec![]; BASE],
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % BASE
    }

    pub fn add(&mut self, key: i32) {
        let k = Self::hash(key);
        if !self.contains(key) {
            self.s[k].push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        let k = Self::hash(key);
        if let Some(p) = self.s[k].iter().position(|e| *e == key) {
            self.s[k].remove(p);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        let k = Self::hash(key);
        self.s[k].contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut s = MyHashSet::new();
        s.add(1);
        s.add(2);
        assert!(s.contains(1));
        assert!(!s.contains(3));
        s.add(2);
        assert!(s.contains(2));
        s.remove(2);
        assert!(!s.contains(2));
    }
}
