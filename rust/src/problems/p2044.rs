pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; 1 << 17];
        dp[0] = 1;
        let mut max = 0;

        for n in nums {
            for i in (0..=max).rev() {
                dp[i | n as usize] += dp[i];
            }
            max |= n as usize;
        }

        dp[max]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_max_or_subsets(vec![3, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(7, Solution::count_max_or_subsets(vec![2, 2, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::count_max_or_subsets(vec![3, 2, 1, 5]));
    }
}
