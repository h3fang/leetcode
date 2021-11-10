use std::collections::{HashMap, HashSet};

pub struct Solution;
struct Trie {
    next: [Option<Box<Trie>>; 26],
    count: i32,
}

impl Trie {
    fn new() -> Self {
        const VAL: Option<Box<Trie>> = None;
        Self {
            next: [VAL; 26],
            count: 0,
        }
    }

    fn insert(&mut self, w: &[u8]) {
        if w.is_empty() {
            self.count += 1;
        } else {
            let c = (w[0] - b'a') as usize;
            if self.next[c].is_none() {
                self.next[c] = Some(Box::new(Trie::new()));
            }
            if let Some(n) = &mut self.next[c] {
                n.insert(&w[1..]);
            }
        }
    }

    fn search(&self, s: &[u8], first: u8) -> i32 {
        if s.is_empty() {
            self.count
        } else {
            let b = s[0];
            let mut r = 0;
            if b != first {
                r += self.search(&s[1..], first);
            }
            match &self.next[(b - b'a') as usize] {
                Some(next) => r + next.search(&s[1..], first),
                None => r,
            }
        }
    }
}

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        fn count(w: &str) -> u32 {
            let mut mask = 0u32;
            for b in w.as_bytes() {
                mask |= 1 << (b - b'a');
            }
            mask
        }

        let mut ws = HashMap::new();
        for w in &words {
            let c = count(w);
            if c.count_ones() <= 7 {
                *ws.entry(c).or_insert(0) += 1;
            }
        }

        puzzles
            .iter()
            .map(|p| {
                let first = 1 << (p.as_bytes()[0] - b'a');
                let mut result = ws.get(&first).cloned().unwrap_or(0);
                let mask = count(&p[1..]);
                let mut submask = mask;
                while submask > 0 {
                    result += ws.get(&(first | submask)).cloned().unwrap_or(0);
                    submask = mask & (submask - 1);
                }
                result
            })
            .collect()
    }

    pub fn find_num_of_valid_words_trie(mut words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut trie = Trie::new();
        for w in &mut words {
            let w = w.as_bytes().iter().cloned().collect::<HashSet<_>>();
            let mut w = w.iter().cloned().collect::<Vec<_>>();
            w.sort_unstable();
            trie.insert(&w);
        }

        puzzles
            .iter()
            .map(|p| {
                let first = p.as_bytes()[0];
                let mut p = p.as_bytes().to_vec();
                p.sort_unstable();
                trie.search(&p, first)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["aaaa", "asas", "able", "ability", "actt", "actor", "access"];
        let words = words.into_iter().map(|w| w.to_string()).collect::<Vec<_>>();
        let puzzles = [
            "aboveyz", "abrodyz", "abslute", "absoryz", "actresz", "gaswxyz",
        ];
        let puzzles = puzzles
            .into_iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            vec![1, 1, 3, 2, 4, 0],
            Solution::find_num_of_valid_words_trie(words.clone(), puzzles.clone())
        );
        assert_eq!(
            vec![1, 1, 3, 2, 4, 0],
            Solution::find_num_of_valid_words(words, puzzles)
        );
    }

    #[test]
    fn case2() {
        let words = ["apple", "pleas", "please"];
        let words = words.into_iter().map(|w| w.to_string()).collect::<Vec<_>>();
        let puzzles = ["aelwxyz", "aelpxyz", "aelpsxy", "saelpxy", "xaelpsy"];
        let puzzles = puzzles
            .into_iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            vec![0, 1, 3, 2, 0],
            Solution::find_num_of_valid_words_trie(words.clone(), puzzles.clone())
        );
        assert_eq!(
            vec![0, 1, 3, 2, 0],
            Solution::find_num_of_valid_words(words, puzzles)
        );
    }
}
