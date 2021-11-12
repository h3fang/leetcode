pub struct Solution;

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for i in (1..n).rev() {
            for j in i + 1..=n {
                let mut min = i32::MAX;
                for k in i..j {
                    min = min.min(k as i32 + dp[i][k - 1].max(dp[k + 1][j]));
                }
                dp[i][j] = min;
            }
        }
        dp[1][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(16, Solution::get_money_amount(10));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::get_money_amount(1));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::get_money_amount(2));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::get_money_amount(3));
    }

    #[test]
    fn case5() {
        assert_eq!(4, Solution::get_money_amount(4));
    }
}
