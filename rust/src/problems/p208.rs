use std::str::Chars;

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
        self.insert_impl(word.chars());
    }

    fn insert_impl(&mut self, mut word: Chars) {
        match word.next() {
            Some(c) => {
                let c = (c as u8 - b'a') as usize;
                match self.next.get_mut(c).unwrap() {
                    Some(ref mut t) => {
                        t.insert_impl(word);
                    }
                    None => {
                        let mut t = Trie::new();
                        t.insert_impl(word);
                        self.next[c] = Some(Box::new(t));
                    }
                }
            }
            None => self.is_end = true,
        }
    }

    pub fn search(&self, word: String) -> bool {
        self.search_impl(word.chars())
    }

    fn search_impl(&self, mut word: Chars) -> bool {
        match word.next() {
            Some(c) => {
                let c = (c as u8 - b'a') as usize;
                if let Some(trie) = &self.next[c] {
                    trie.search_impl(word)
                } else {
                    false
                }
            }
            None => self.is_end,
        }
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        self.starts_with_impl(prefix.chars())
    }

    fn starts_with_impl(&self, mut prefix: Chars) -> bool {
        match prefix.next() {
            Some(c) => {
                let c = (c as u8 - b'a') as usize;
                if let Some(trie) = &self.next[c] {
                    trie.starts_with_impl(prefix)
                } else {
                    false
                }
            }
            None => true,
        }
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
        assert_eq!(true, obj.search("apple".to_string()));
        assert_eq!(false, obj.search("app".to_string()));
        assert_eq!(true, obj.starts_with("app".to_string()));
        obj.insert("app".to_string());
        assert_eq!(true, obj.search("app".to_string()));
    }
}
