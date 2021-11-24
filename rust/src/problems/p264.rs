pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        let mut two = 1;
        let mut three = 1;
        let mut five = 1;
        dp[1] = 1;
        for i in 2..=n {
            let m = (2 * dp[two]).min((3 * dp[three]).min(5 * dp[five]));
            dp[i] = m;
            if m == 2 * dp[two] {
                two += 1;
            }
            if m == 3 * dp[three] {
                three += 1;
            }
            if m == 5 * dp[five] {
                five += 1;
            }
        }
        dp[n as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::nth_ugly_number(10));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::nth_ugly_number(1));
    }

    #[test]
    fn case3() {
        assert_eq!(1536, Solution::nth_ugly_number(100));
    }
}
