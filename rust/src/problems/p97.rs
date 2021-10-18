pub struct Solution;

impl Solution {
    pub fn is_interleave_recursive(s1: String, s2: String, s3: String) -> bool {
        fn recursive(s1: &str, s2: &str, s3: &str, dp: &mut [Vec<u8>]) -> bool {
            if s1.is_empty() && s2.is_empty() && s3.is_empty() {
                true
            } else {
                if dp[s1.len()][s2.len()] > 0 {
                    return dp[s1.len()][s2.len()] == 1;
                }
                let mut r = false;
                if !s1.is_empty() && s1[..1] == s3[..1] {
                    r = r || recursive(&s1[1..], s2, &s3[1..], dp);
                }
                if !s2.is_empty() && s2[..1] == s3[..1] {
                    r = r || recursive(s1, &s2[1..], &s3[1..], dp);
                }

                dp[s1.len()][s2.len()] = if r { 1 } else { 2 };

                r
            }
        }
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        recursive(&s1, &s2, &s3, &mut dp)
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let m = s1.len();
        let n = s2.len();
        let mut dp = vec![false; n + 1];
        for i in 0..=m {
            for j in 0..=n {
                if i == 0 && j == 0 {
                    dp[0] = true;
                } else if i == 0 {
                    dp[j] = s2[j - 1..j] == s3[i + j - 1..i + j] && dp[j - 1];
                } else if j == 0 {
                    dp[j] = s1[i - 1..i] == s3[i + j - 1..i + j] && dp[j];
                } else {
                    dp[j] = (s2[j - 1..j] == s3[i + j - 1..i + j] && dp[j - 1])
                        || (s1[i - 1..i] == s3[i + j - 1..i + j] && dp[j]);
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
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        assert_eq!(true, Solution::is_interleave(s1, s2, s3));
    }

    #[test]
    fn case2() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        assert_eq!(false, Solution::is_interleave(s1, s2, s3));
    }

    #[test]
    fn case3() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        assert_eq!(true, Solution::is_interleave(s1, s2, s3));
    }

    #[test]
    fn case4() {
        let s1 = "a".to_string();
        let s2 = "b".to_string();
        let s3 = "a".to_string();
        assert_eq!(false, Solution::is_interleave(s1, s2, s3));
    }
}
