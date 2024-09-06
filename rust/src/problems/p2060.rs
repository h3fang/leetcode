use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn possibly_equals(s1: String, s2: String) -> bool {
        let m = s1.len();
        let n = s2.len();
        let mut dp = vec![vec![HashSet::<i32>::new(); n + 1]; m + 1];
        dp[0][0].insert(0);

        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();

        for i in 0..=m {
            for j in 0..=n {
                let dp_ij = dp[i][j].clone();

                // s1[i] is digit
                let mut num = 0;
                for (k, b) in s1.iter().enumerate().take((i + 3).min(m)).skip(i) {
                    if b.is_ascii_digit() {
                        num = num * 10 + (b - b'0') as i32;
                        for diff in &dp_ij {
                            dp[k + 1][j].insert(diff + num);
                        }
                    } else {
                        break;
                    }
                }

                // s2[j] is digit
                let mut num = 0;
                for (k, b) in s2.iter().enumerate().take((j + 3).min(n)).skip(j) {
                    if b.is_ascii_digit() {
                        num = num * 10 + (b - b'0') as i32;
                        for diff in &dp_ij {
                            dp[i][k + 1].insert(diff - num);
                        }
                    } else {
                        break;
                    }
                }

                // s1[i] is alphabetic
                if i < m && s1[i].is_ascii_alphabetic() {
                    for &diff in &dp_ij {
                        if diff < 0 {
                            dp[i + 1][j].insert(diff + 1);
                        }
                    }
                }

                // s2[j] is alphabetic
                if j < n && s2[j].is_ascii_alphabetic() {
                    for &diff in &dp_ij {
                        if diff > 0 {
                            dp[i][j + 1].insert(diff - 1);
                        }
                    }
                }

                // s1[i] and s2[j] are alphabetic
                if i < m
                    && s1[i].is_ascii_alphabetic()
                    && j < n
                    && s2[j].is_ascii_alphabetic()
                    && s1[i] == s2[j]
                    && dp_ij.contains(&0)
                {
                    dp[i + 1][j + 1].insert(0);
                }
            }
        }

        dp[m][n].contains(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s1 = "internationalization".into();
        let s2 = "i18n".into();
        assert!(Solution::possibly_equals(s1, s2));
    }

    #[test]
    fn case2() {
        let s1 = "l123e".into();
        let s2 = "44".into();
        assert!(Solution::possibly_equals(s1, s2));
    }

    #[test]
    fn case3() {
        let s1 = "a5b".into();
        let s2 = "c5b".into();
        assert!(!Solution::possibly_equals(s1, s2));
    }

    #[test]
    fn case4() {
        let s1 = "112s".into();
        let s2 = "g841".into();

        assert!(Solution::possibly_equals(s1, s2));
    }

    #[test]
    fn case5() {
        let s1 = "ab".into();
        let s2 = "a2".into();

        assert!(!Solution::possibly_equals(s1, s2));
    }
}
