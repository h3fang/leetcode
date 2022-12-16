pub struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        fn zero_one(s: &[u8]) -> (usize, usize) {
            let z = s.iter().filter(|b| **b == b'0').count();
            (z, s.len() - z)
        }

        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for s in &strs {
            let (z, o) = zero_one(s.as_bytes());
            for j in (z..m + 1).rev() {
                for k in (o..n + 1).rev() {
                    dp[j][k] = dp[j][k].max(dp[j - z][k - o] + 1);
                }
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        let m = 5;
        let n = 3;
        assert_eq!(4, Solution::find_max_form(strs, m, n));
    }
}
