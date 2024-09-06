pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut m = HashMap::new();
        for c in s.chars() {
            *m.entry(c).or_insert(0) += 1;
        }
        for c in t.chars() {
            let e = m.entry(c).or_insert(0);
            *e -= 1;
            if *e < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_anagram("anagram".into(), "nagaram".into()));
    }
}
