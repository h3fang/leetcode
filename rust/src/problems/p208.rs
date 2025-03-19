pub struct Trie {
    next: Vec<Option<Box<Trie>>>,
    is_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            next: (0..26).map(|_| None).collect(),
            is_end: false,
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut t = self;
        for &b in word.as_bytes() {
            let i = (b - b'a') as usize;
            t = t.next[i].get_or_insert_default();
        }
        t.is_end = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut t = self;
        for &b in word.as_bytes() {
            let i = (b - b'a') as usize;
            if let Some(trie) = &t.next[i] {
                t = trie;
            } else {
                return false;
            }
        }
        t.is_end
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut t = self;
        for &b in prefix.as_bytes() {
            let i = (b - b'a') as usize;
            if let Some(trie) = &t.next[i] {
                t = trie;
            } else {
                return false;
            }
        }
        true
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut obj = Trie::new();
        obj.insert("apple".to_string());
        assert!(obj.search("apple".to_string()));
        assert!(!obj.search("app".to_string()));
        assert!(obj.starts_with("app".to_string()));
        obj.insert("app".to_string());
        assert!(obj.search("app".to_string()));
    }
}
