pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        if s1.len() != s2.len() {
            return false;
        }
        let mut m = HashMap::new();
        for c in s1.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        for c in s2.chars() {
            *m.entry(c).or_insert(0) -= 1;
        }
        m.values().all(|v| *v == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_permutation(
            "abc".to_string(),
            "bca".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_permutation(
            "abc".to_string(),
            "bad".to_string()
        ));
    }
}
