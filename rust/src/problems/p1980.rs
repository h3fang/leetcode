pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len();
        let nums: HashSet<u32> = nums
            .into_iter()
            .map(|n| u32::from_str_radix(&n, 2).unwrap())
            .collect();
        for i in 0..(1 << n) {
            if !nums.contains(&i) {
                return format!("{i:00$b}", n);
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(nums: &[&str]) {
        let nums = nums.iter().map(|n| n.to_string()).collect::<Vec<_>>();
        let result = Solution::find_different_binary_string(nums.clone());
        assert!(result.len() == nums.len());
        assert!(u16::from_str_radix(&result, 2).is_ok());
        assert!(!nums.contains(&result));
    }

    #[test]
    fn case1() {
        check(&["01", "10"]);
    }

    #[test]
    fn case2() {
        check(&["00", "01"]);
    }

    #[test]
    fn case3() {
        check(&["111", "011", "001"]);
    }

    #[test]
    fn case4() {
        check(&["001011", "111110", "010110", "010010", "101111", "011001"]);
    }
}
