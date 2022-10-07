pub struct Solution;

impl Solution {
    pub fn delete_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut lcp = vec![vec![0; n + 1]; n + 1];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if s[i] == s[j] {
                    lcp[i][j] = lcp[i + 1][j + 1] + 1;
                }
            }
        }
        let mut dp = vec![0; n];
        dp[n - 1] = 1;
        for i in (0..n - 1).rev() {
            let mut max = 1;
            for (j, d) in dp.iter().enumerate().take(n).skip(i + 1) {
                if j + (j - i) > n {
                    break;
                }
                if lcp[i][j] >= j - i {
                    max = max.max(1 + d);
                }
            }
            dp[i] = max;
        }
        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::delete_string("abcabcdabc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::delete_string("aaabaab".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::delete_string("aaaaa".to_string()));
    }
}
