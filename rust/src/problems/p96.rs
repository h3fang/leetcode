pub struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        for i in 1..=n as usize {
            for j in 0..=i - 1 {
                dp[i] += dp[j] * dp[i - 1 - j];
            }
        }
        dp[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::num_trees(1));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::num_trees(2));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::num_trees(3));
    }

    #[test]
    fn case4() {
        assert_eq!(14, Solution::num_trees(4));
    }
}
