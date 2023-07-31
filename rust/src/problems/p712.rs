pub struct Solution;

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        let (m, n) = (s1.len(), s2.len());
        let mut f = vec![vec![0; n + 1]; m + 1];
        for i in 1..m + 1 {
            f[i][0] = f[i - 1][0] + s1[i - 1] as i32;
        }
        for j in 1..n + 1 {
            f[0][j] = f[0][j - 1] + s2[j - 1] as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                if s1[i - 1] == s2[j - 1] {
                    f[i][j] = f[i - 1][j - 1];
                } else {
                    f[i][j] = (f[i - 1][j] + s1[i - 1] as i32).min(f[i][j - 1] + s2[j - 1] as i32);
                }
            }
        }
        f[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            231,
            Solution::minimum_delete_sum("sea".to_string(), "eat".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            403,
            Solution::minimum_delete_sum("delete".to_string(), "leet".to_string())
        );
    }
}
