#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn insert(&mut self, w: &[u8]) {
        let mut t = self;
        for b in w {
            let i = (b - b'a') as usize;
            t = t.next[i].get_or_insert_with(Default::default);
        }
        t.is_end = true;
    }
}

#[derive(Default)]
pub struct MagicDictionary {
    trie: Trie,
}

impl MagicDictionary {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_dict(&mut self, dictionary: Vec<String>) {
        for d in dictionary {
            self.trie.insert(d.as_bytes());
        }
    }

    pub fn search(&self, search_word: String) -> bool {
        fn dfs(t: &Trie, word: &[u8], modified: bool) -> bool {
            if word.is_empty() {
                return modified && t.is_end;
            }
            let i = (word[0] - b'a') as usize;
            if let Some(t) = &t.next[i]
                && dfs(t, &word[1..], modified)
            {
                return true;
            }
            if !modified {
                let o = word[0];
                for b in b'a'..=b'z' {
                    if o == b {
                        continue;
                    }
                    let i = (b - b'a') as usize;
                    if let Some(t) = &t.next[i]
                        && dfs(t, &word[1..], true)
                    {
                        return true;
                    }
                }
            }
            false
        }

        dfs(&self.trie, search_word.as_bytes(), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut md = MagicDictionary::new();
        md.build_dict(vec!["hello".to_string(), "leetcode".to_string()]);
        assert!(!md.search("hello".into()));
        assert!(md.search("hhllo".into()));
        assert!(!md.search("hell".into()));
        assert!(!md.search("leetcoded".into()));
    }
}
