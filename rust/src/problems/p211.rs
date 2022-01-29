#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn insert(&mut self, word: &[u8]) {
        if let Some(b) = word.first() {
            let i = (b - b'a') as usize;
            if self.next[i].is_none() {
                self.next[i] = Some(Box::new(Trie::default()));
            }
            if let Some(t) = &mut self.next[i] {
                t.insert(&word[1..]);
            }
        } else {
            self.is_end = true;
        }
    }

    fn search(&self, word: &[u8]) -> bool {
        if let Some(&b) = word.first() {
            if b == b'.' {
                self.next.iter().any(|t| {
                    if let Some(t) = t {
                        t.search(&word[1..])
                    } else {
                        false
                    }
                })
            } else {
                let i = (b - b'a') as usize;
                if let Some(t) = &self.next[i] {
                    t.search(&word[1..])
                } else {
                    false
                }
            }
        } else {
            self.is_end
        }
    }
}

#[derive(Default)]
pub struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_word(&mut self, word: String) {
        self.trie.insert(word.as_bytes());
    }

    pub fn search(&self, word: String) -> bool {
        self.trie.search(word.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        assert_eq!(false, wd.search("pad".to_string()));
        assert_eq!(true, wd.search("bad".to_string()));
        assert_eq!(true, wd.search(".ad".to_string()));
        assert_eq!(true, wd.search("b..".to_string()));
    }
}
