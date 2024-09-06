pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let n_s = s.len();
        let n_p = p.len();
        let s = s.as_bytes();
        let p = p.as_bytes();
        let mut dp = vec![vec![false; n_p + 1]; n_s + 1];
        dp[0][0] = true;
        for j in 1..n_p + 1 {
            if p[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }
        for i in 1..n_s + 1 {
            for j in 1..n_p + 1 {
                match p[j - 1] {
                    b'.' => dp[i][j] = dp[i - 1][j - 1],
                    b'*' => {
                        if dp[i][j - 2] {
                            dp[i][j] = true;
                        } else {
                            dp[i][j] = (p[j - 2] == b'.' || s[i - 1] == p[j - 2])
                                && (dp[i - 1][j] || dp[i][j - 2]);
                        }
                    }
                    b => dp[i][j] = dp[i - 1][j - 1] && b == s[i - 1],
                }
            }
        }

        dp[n_s][n_p]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "aa".to_string();
        let p = "a".to_string();
        assert!(!Solution::is_match(s, p));
    }

    #[test]
    fn case2() {
        let s = "aa".to_string();
        let p = "a*".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn case3() {
        let s = "ab".to_string();
        let p = ".*".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn case4() {
        let s = "aab".to_string();
        let p = "c*a*b".to_string();
        assert!(Solution::is_match(s, p));
    }

    #[test]
    fn case5() {
        let s = "mississippi".to_string();
        let p = "mis*is*p*.".to_string();
        assert!(!Solution::is_match(s, p));
    }
}
