use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut r = 0i32;
        let mut dp = vec![HashMap::new(); n];
        for i in 1..n {
            for j in 0..i {
                let delta = nums[i] - nums[j];
                let count = if dp[j].contains_key(&delta) {
                    dp[j][&delta] + 1
                } else {
                    2
                };
                let e = dp[i].entry(delta).or_insert(0);
                *e = (*e).max(count);

                r = r.max(*e);
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![3, 6, 9, 12];
        let expected = 4;
        assert_eq!(expected, Solution::longest_arith_seq_length(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![9, 4, 7, 2, 10];
        let expected = 3;
        assert_eq!(expected, Solution::longest_arith_seq_length(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![20, 1, 15, 3, 10, 5, 8];
        let expected = 4;
        assert_eq!(expected, Solution::longest_arith_seq_length(nums));
    }
}
