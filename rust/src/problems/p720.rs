pub struct Solution;

use std::cmp::Ordering;
use std::collections::HashSet;

impl Solution {
    pub fn longest_word(mut words: Vec<String>) -> String {
        let mut result = "";
        let mut candidates = HashSet::new();
        candidates.insert("");

        words.sort_by(|a, b| match a.len().cmp(&b.len()) {
            Ordering::Equal => b.cmp(a),
            o => o,
        });

        for w in words.iter() {
            if candidates.contains(&w[..w.len() - 1]) {
                result = w;
                candidates.insert(w);
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["w", "wo", "wor", "worl", "world"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!("world", Solution::longest_word(words));
    }

    #[test]
    fn case2() {
        let words = ["a", "banana", "app", "appl", "ap", "apply", "apple"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!("apple", Solution::longest_word(words));
    }
}
