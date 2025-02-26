pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        fn f<'a>(
            s: &'a str,
            words: &HashSet<&str>,
            cache: &mut HashMap<&'a str, Vec<String>>,
        ) -> Vec<String> {
            if let Some(r) = cache.get(s) {
                return r.clone();
            }
            let mut result = vec![];
            if words.contains(s) {
                result.push(s.to_string());
            }
            for i in 1..=s.len() {
                let w = &s[..i];
                if words.contains(w) {
                    for r in f(&s[i..], words, cache) {
                        result.push(format!("{w} {r}"));
                    }
                }
            }

            cache.insert(s, result.clone());
            result
        }

        let words = word_dict.iter().map(|w| w.as_str()).collect::<HashSet<_>>();
        let mut cache = HashMap::new();
        f(&s, &words, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "catsanddog".to_string();
        let word_dict = ["cat", "cats", "and", "sand", "dog"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let mut expected = ["cats and dog", "cat sand dog"]
            .iter()
            .map(|w| w.to_string())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut r = Solution::word_break(s, word_dict);
        r.sort_unstable();
        assert_eq!(expected, r);
    }

    #[test]
    fn case2() {
        let s = "pineapplepenapple".to_string();
        let word_dict = ["apple", "pen", "applepen", "pine", "pineapple"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let mut expected = [
            "pine apple pen apple",
            "pineapple pen apple",
            "pine applepen apple",
        ]
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut r = Solution::word_break(s, word_dict);
        r.sort_unstable();
        assert_eq!(expected, r);
    }

    #[test]
    fn case3() {
        let s = "catsandog".to_string();
        let word_dict = ["cats", "dog", "sand", "and", "cat"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let r = Solution::word_break(s, word_dict);
        assert!(r.is_empty());
    }
}
