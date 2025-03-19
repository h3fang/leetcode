pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut s = HashSet::new();
        let (mut sum, mut max) = (0, nums[0]);
        for &x in &nums {
            max = max.max(x);
            if x > 0 && s.insert(x) {
                sum += x;
            }
        }
        if sum > 0 { sum } else { max }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::max_sum(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_sum(vec![1, 1, 0, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::max_sum(vec![1, 2, -1, -2, 1, 0, -1]));
    }
}
