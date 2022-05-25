pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![0; n];
        let mut result = 0;
        for i in 1..n {
            if s[i] == b'(' {
                continue;
            }
            if s[i - 1] == b'(' {
                dp[i] = if i >= 2 { dp[i - 2] } else { 0 } + 2;
            } else if i > dp[i - 1] && s[i - 1 - dp[i - 1]] == b'(' {
                let prev = if i >= 2 + dp[i - 1] {
                    dp[i - 2 - dp[i - 1]]
                } else {
                    0
                };
                dp[i] = dp[i - 1] + prev + 2;
            }
            result = result.max(dp[i]);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_valid_parentheses("(()".into()));
    }
}
