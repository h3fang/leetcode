pub struct Solution;

use std::{cmp::Reverse, collections::HashMap};

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut f = HashMap::new();
        for w in &words {
            *f.entry(w.as_str()).or_insert(0) += 1;
        }
        let mut f = f
            .into_iter()
            .map(|(k, v)| (Reverse(v), k))
            .collect::<Vec<_>>();
        f.sort_unstable();
        f.into_iter()
            .take(k as usize)
            .map(|e| e.1.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["i", "love", "leetcode", "i", "love", "coding"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let k = 2;
        let expected = ["i", "love"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::top_k_frequent(words, k));
    }

    #[test]
    fn case2() {
        let words = [
            "the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect();
        let k = 4;
        let expected = ["the", "is", "sunny", "day"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::top_k_frequent(words, k));
    }
}
