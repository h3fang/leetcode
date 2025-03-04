pub struct Solution;

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let (n, s) = (s.len(), s.as_bytes());
        let mut f = vec![vec![false; n]; n];
        for i in (0..n).rev() {
            f[i][i] = true;
            for j in i + 1..n {
                f[i][j] = if s[i] == s[j] {
                    j - i == 1 || f[i + 1][j - 1]
                } else {
                    false
                };
            }
        }
        for i in 0..n - 2 {
            if !f[0][i] {
                continue;
            }
            for j in i + 1..n - 1 {
                if f[i + 1][j] && f[j + 1][n - 1] {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_partitioning("abcbdd".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_partitioning("bcbddxy".to_string()));
    }
}
