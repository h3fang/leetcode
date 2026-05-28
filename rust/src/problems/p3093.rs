pub struct Solution;

struct Trie {
    next: [Option<Box<Trie>>; 26],
    index: usize,
    len: usize,
}

impl Default for Trie {
    fn default() -> Self {
        Self {
            next: Default::default(),
            index: 0,
            len: usize::MAX,
        }
    }
}

impl Trie {
    fn insert(&mut self, word: &[u8], index: usize) {
        if word.len() < self.len {
            self.len = word.len();
            self.index = index;
        }

        let mut t = self;
        for &b in word.iter().rev() {
            t = t.next[(b - b'a') as usize].get_or_insert_default();
            if t.len > word.len() {
                t.len = word.len();
                t.index = index;
            }
        }
    }

    fn search(&self, word: &[u8]) -> usize {
        let mut t = self;
        for &b in word.iter().rev() {
            match &t.next[(b - b'a') as usize] {
                Some(next) => t = next,
                None => break,
            }
        }
        t.index
    }
}

impl Solution {
    pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::default();
        for (i, w) in words_container.into_iter().enumerate() {
            trie.insert(w.as_bytes(), i);
        }
        words_query
            .into_iter()
            .map(|w| trie.search(w.as_bytes()) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words_container = ["abcd", "bcd", "xbcd"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let words_query = ["cd", "bcd", "xyz"].iter().map(|w| w.to_string()).collect();
        assert_eq!(
            vec![1, 1, 1],
            Solution::string_indices(words_container, words_query)
        );
    }

    #[test]
    fn case2() {
        let words_container = ["abcdefgh", "poiuygh", "ghghgh"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let words_query = ["gh", "acbfgh", "acbfegh"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(
            vec![2, 0, 2],
            Solution::string_indices(words_container, words_query)
        );
    }
}
