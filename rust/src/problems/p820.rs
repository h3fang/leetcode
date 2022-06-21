pub struct Solution;

use std::collections::HashSet;

#[derive(Default)]
struct SuffixTrie {
    next: [Option<Box<SuffixTrie>>; 26],
    n_children: i8,
}

impl SuffixTrie {
    fn insert(&mut self, w: &[u8]) {
        let mut t = self;
        for c in w.iter().rev() {
            let i = (c - b'a') as usize;
            if t.next[i].is_none() {
                t.n_children += 1;
                t.next[i] = Some(Box::new(SuffixTrie::default()));
            }
            t = t.next[i].as_mut().unwrap();
        }
    }

    fn search(&self, w: &[u8]) -> bool {
        let mut t = self;
        for c in w.iter().rev() {
            let i = (c - b'a') as usize;
            if let Some(n) = &t.next[i] {
                t = n;
            } else {
                return false;
            }
        }
        t.n_children == 0
    }
}

impl Solution {
    pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
        let mut trie = SuffixTrie::default();
        for w in &words {
            trie.insert(w.as_bytes());
        }
        let mut result = 0;
        let mut set: HashSet<&str> = HashSet::new();
        for w in &words {
            if set.contains(w.as_str()) {
                continue;
            }
            set.insert(w.as_str());
            if trie.search(w.as_bytes()) {
                result += w.as_bytes().len() + 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["time", "me", "bell"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(10, Solution::minimum_length_encoding(words));
    }
}
