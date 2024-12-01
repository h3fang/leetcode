pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut s = HashSet::with_capacity(arr.len());
        for x in arr {
            if s.contains(&(2 * x)) || (x % 2 == 0 && s.contains(&(x / 2))) {
                return true;
            }
            s.insert(x);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_if_exist(vec![10, 2, 5, 3]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_if_exist(vec![3, 1, 7, 11]));
    }
}
