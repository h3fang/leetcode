pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut m = HashMap::with_capacity(words.len());
        let mut result = 0;
        for w in words {
            let mut mask = 0;
            for &b in w.as_bytes() {
                mask |= 1 << (b - b'a');
            }
            result += m.get(&mask).cloned().unwrap_or(0);
            *m.entry(mask).or_insert(0) += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["aba", "aabb", "abcd", "bac", "aabc"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(2, Solution::similar_pairs(words));
    }

    #[test]
    fn case2() {
        let words = ["aabb", "ab", "ba"].iter().map(|w| w.to_string()).collect();
        assert_eq!(3, Solution::similar_pairs(words));
    }

    #[test]
    fn case3() {
        let words = ["nba", "cba", "dba"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        assert_eq!(0, Solution::similar_pairs(words));
    }
}
