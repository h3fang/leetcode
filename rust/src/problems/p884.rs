use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words = HashMap::new();
        for w in s1.split_ascii_whitespace() {
            *words.entry(w).or_insert(0) += 1;
        }
        for w in s2.split_ascii_whitespace() {
            *words.entry(w).or_insert(0) += 1;
        }
        let mut result = vec![];
        for (w, &c) in words.iter() {
            if c == 1 {
                result.push(w.to_string());
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
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        let expected = ["sweet", "sour"];
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::uncommon_from_sentences(s1, s2);
        result.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        let expected = ["sweet", "sour"];
        let mut expected = expected.iter().map(|w| w.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::uncommon_from_sentences(s1, s2);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
