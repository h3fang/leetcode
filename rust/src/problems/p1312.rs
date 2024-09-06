pub struct Solution;

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let r = s.chars().rev().collect::<String>();
        let s = s.as_bytes();
        let r = r.as_bytes();
        let n = s.len();
        let mut dp = vec![0; n + 1];
        let mut dp_prev = vec![0; n + 1];
        for i in 0..=n {
            for j in 0..=n {
                dp[j] = if i == 0 || j == 0 {
                    0
                } else if s[i - 1] == r[j - 1] {
                    1 + dp_prev[j - 1]
                } else {
                    dp_prev[j].max(dp[j - 1])
                };
            }
            dp_prev.clone_from(&dp);
        }
        n as i32 - dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::min_insertions("zzazz".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::min_insertions("mbadm".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::min_insertions("leetcode".to_string()));
    }
}
