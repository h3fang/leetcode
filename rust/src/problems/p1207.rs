pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut f = HashMap::new();
        arr.iter().for_each(|&n| *f.entry(n).or_insert(0) += 1);
        let mut s = HashSet::new();
        f.values().all(|&e| {
            if s.contains(&e) {
                false
            } else {
                s.insert(e);
                true
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::unique_occurrences(vec![1, 2]));
    }

    #[test]
    fn case3() {
        assert!(Solution::unique_occurrences(vec![
            -3, 0, 1, -3, 1, 1, 1, -3, 10, 0
        ]));
    }
}
