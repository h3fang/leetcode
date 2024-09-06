use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let dict: HashSet<&str> = HashSet::from_iter(word_dict.iter().map(|e| e.as_str()));
        let n = s.len();
        let mut dp = vec![false; n + 1];
        dp[n] = true;

        for i in (0..n).rev() {
            for j in i..=n {
                if dp[j] && dict.contains(&s[i..j]) {
                    dp[i] = true;
                }
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "leetcode".to_string();
        let word_dict = ["leet", "code"];
        let word_dict = word_dict.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(Solution::word_break(s, word_dict));
    }

    #[test]
    fn case2() {
        let s = "applepenapple".to_string();
        let word_dict = ["apple", "pen"];
        let word_dict = word_dict.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(Solution::word_break(s, word_dict));
    }

    #[test]
    fn case3() {
        let s = "catsandog".to_string();
        let word_dict = ["cats", "dog", "sand", "and", "cat"];
        let word_dict = word_dict.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(!Solution::word_break(s, word_dict));
    }
}
