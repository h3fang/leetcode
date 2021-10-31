pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = s.len();
        let mut dp = vec![0; n + 1];
        dp[n] = 1;
        dp[n - 1] = if bytes[n - 1] == b'0' { 0 } else { 1 };

        for i in (0..n - 1).rev() {
            match bytes[i] {
                b'0' => dp[i] = 0,
                b'1' => dp[i] = dp[i + 1] + dp[i + 2],
                b'2' if bytes[i + 1] <= b'6' => dp[i] = dp[i + 1] + dp[i + 2],
                _ => dp[i] = dp[i + 1],
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::num_decodings("12".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::num_decodings("226".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::num_decodings("0".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::num_decodings("06".to_string()));
    }
}
