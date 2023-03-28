pub struct Solution;

impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let s = str1.as_bytes();
        let t = str2.as_bytes();
        let m = s.len();
        let n = t.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        (0..m).for_each(|i| dp[i][n] = m - i);
        dp[m].iter_mut().enumerate().for_each(|(j, e)| *e = n - j);
        for (i, &a) in s.iter().enumerate().rev() {
            for (j, &b) in t.iter().enumerate().rev() {
                dp[i][j] = if a == b {
                    dp[i + 1][j + 1] + 1
                } else {
                    dp[i + 1][j].min(dp[i][j + 1]) + 1
                };
            }
        }
        let mut result = String::with_capacity(dp[0][0]);
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            if s[i] == t[j] {
                result.push(s[i] as char);
                i += 1;
                j += 1;
            } else if dp[i + 1][j] == dp[i][j] - 1 {
                result.push(s[i] as char);
                i += 1;
            } else {
                result.push(t[j] as char);
                j += 1;
            }
        }
        if i < m {
            result.push_str(&str1[i..]);
        } else if j < n {
            result.push_str(&str2[j..]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "cabac",
            Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string())
        );
    }
}
