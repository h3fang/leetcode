pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        let mut k = 1;
        while k * k <= n {
            dp[(k * k) as usize] = 1;
            k += 1;
        }
        if dp[n as usize] == 1 {
            return 1;
        }

        for i in 1..=n as usize {
            if dp[i] == 0 {
                let mut min = i32::MAX;
                let mut k = 1;
                while k * k <= i {
                    min = min.min(dp[i - k * k] + 1);
                    if min == 2 {
                        break;
                    }
                    k += 1;
                }
                dp[i] = min;
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
        assert_eq!(3, Solution::num_squares(12));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::num_squares(13));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::num_squares(4));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::num_squares(53));
    }
}
