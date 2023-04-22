pub struct Solution;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut r = 0i32;
        let mut dp = vec![[-1; 1001]; n];
        for i in 1..n {
            for j in 0..i {
                let delta = nums[i] - nums[j];
                let d = (delta + 500) as usize;
                let count = if dp[j][d] != -1 { dp[j][d] + 1 } else { 2 };
                dp[i][d] = dp[i][d].max(count);
                r = r.max(dp[i][d]);
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
