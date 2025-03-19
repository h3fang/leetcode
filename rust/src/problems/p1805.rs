pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        word.split(|c: char| c.is_alphabetic())
            .filter(|w| !w.is_empty())
            .map(|w| {
                let w = w.trim_start_matches('0');
                if w.is_empty() { "0" } else { w }
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::num_different_integers("a123bc34d8ef34".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::num_different_integers("leet1234code234".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::num_different_integers("a1b01c001".to_string()));
    }
}
