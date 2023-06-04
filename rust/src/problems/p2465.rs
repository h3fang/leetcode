pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn distinct_averages(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let s = (0..n / 2)
            .map(|i| nums[i] + nums[n - 1 - i])
            .collect::<HashSet<_>>();
        s.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::distinct_averages(vec![4, 1, 4, 0, 3, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::distinct_averages(vec![1, 100]));
    }
}
