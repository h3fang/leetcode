use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut w1: HashMap<&str, usize> = HashMap::new();
        for s in &words1 {
            *w1.entry(s.as_str()).or_default() += 1;
        }

        let mut w2: HashMap<&str, usize> = HashMap::new();
        for s in &words2 {
            *w2.entry(s.as_str()).or_default() += 1;
        }

        let mut result = 0;

        for s in &words1 {
            if w1.get(s.as_str()).cloned().unwrap_or(0) == 1
                && w2.get(s.as_str()).cloned().unwrap_or(0) == 1
            {
                result += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words1 = ["leetcode", "is", "amazing", "as", "is"];
        let words1 = words1.iter().map(|s| s.to_string()).collect();
        let words2 = ["amazing", "leetcode", "is"];
        let words2 = words2.iter().map(|s| s.to_string()).collect();
        assert_eq!(2, Solution::count_words(words1, words2));
    }

    #[test]
    fn case2() {
        let words1 = ["b", "bb", "bbb"];
        let words1 = words1.iter().map(|s| s.to_string()).collect();
        let words2 = ["a", "aa", "aaa"];
        let words2 = words2.iter().map(|s| s.to_string()).collect();
        assert_eq!(0, Solution::count_words(words1, words2));
    }

    #[test]
    fn case3() {
        let words1 = ["a", "ab"];
        let words1 = words1.iter().map(|s| s.to_string()).collect();
        let words2 = ["a", "a", "a", "ab"];
        let words2 = words2.iter().map(|s| s.to_string()).collect();
        assert_eq!(1, Solution::count_words(words1, words2));
    }
}
