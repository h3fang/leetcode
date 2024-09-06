use std::str::Chars;

struct Trie {
    next: Vec<Option<Box<Trie>>>,
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            next: (0..26).map(|_| None).collect(),
            is_end: false,
        }
    }

    fn insert(&mut self, word: String) {
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
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

pub struct StreamChecker {
    trie: Trie,
    chars: Vec<char>,
}

impl StreamChecker {
    pub fn new(words: Vec<String>) -> Self {
        let mut trie = Trie::new();
        for w in words {
            trie.insert(w.chars().rev().collect());
        }

        Self {
            trie,
            chars: vec![],
        }
    }

    pub fn query(&mut self, letter: char) -> bool {
        self.chars.push(letter);
        let mut t = &self.trie;
        for &c in self.chars.iter().rev() {
            let c = (c as u8 - b'a') as usize;
            if let Some(trie) = &t.next[c] {
                t = trie.as_ref();
                if t.is_end {
                    return true;
                }
            } else {
                return false;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut sc = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
        assert!(!sc.query('a'));
        assert!(!sc.query('b'));
        assert!(!sc.query('c'));
        assert!(sc.query('d'));
        assert!(!sc.query('e'));
        assert!(sc.query('f'));
        assert!(!sc.query('g'));
        assert!(!sc.query('h'));
        assert!(!sc.query('i'));
        assert!(!sc.query('j'));
        assert!(!sc.query('k'));
        assert!(sc.query('l'));
    }
}
