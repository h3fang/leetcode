pub struct Solution;

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n = s1.len();
        if s2.len() != n {
            return false;
        }
        let mut dp = vec![vec![vec![false; n + 1]; n]; n];

        for k in 1..=n {
            for i in 0..=n - k {
                for j in 0..=n - k {
                    if k == 1 {
                        dp[i][j][k] = s1[i..i + 1] == s2[j..j + 1];
                    } else {
                        dp[i][j][k] = (1..k).any(|m| {
                            (dp[i][j][m] && dp[i + m][j + m][k - m])
                                || (dp[i + m][j][k - m] && dp[i][j + k - m][m])
                        });
                    }
                }
            }
        }
        dp[0][0][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::is_scramble("great".to_string(), "rgeat".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            false,
            Solution::is_scramble("abcde".to_string(), "caebd".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            true,
            Solution::is_scramble("ac".to_string(), "ca".to_string())
        );
    }
}
