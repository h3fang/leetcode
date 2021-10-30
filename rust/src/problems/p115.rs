pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let m = s.len();
        let n = t.len();
        if m < n {
            return 0;
        }
        let mut dp = vec![0; n + 1];

        let s = s.as_bytes();
        let t = t.as_bytes();

        dp[0] = 1;
        for i in 1..=m {
            for j in (1..=n).rev() {
                if s[i - 1] == t[j - 1] {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "rabbbit".to_string();
        let t = "rabbit".to_string();
        assert_eq!(3, Solution::num_distinct(s, t));
    }

    #[test]
    fn case2() {
        let s = "babgbag".to_string();
        let t = "bag".to_string();
        assert_eq!(5, Solution::num_distinct(s, t));
    }
}
