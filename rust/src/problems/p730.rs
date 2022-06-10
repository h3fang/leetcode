pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![vec![0; n]; n]; 4];
        for (i, c) in s.iter().enumerate() {
            let x = (c - b'a') as usize;
            dp[x][i][i] = 1;
        }
        for dist in 2..=n {
            for j in dist - 1..n {
                let i = j + 1 - dist;
                for (x, c) in (b'a'..=b'd').enumerate() {
                    if s[i] == c && s[j] == c {
                        dp[x][i][j] = (2 + dp.iter().map(|d| d[i + 1][j - 1]).sum::<i64>()) % MOD;
                    } else if s[i] == c {
                        dp[x][i][j] = dp[x][i][j - 1];
                    } else if s[j] == c {
                        dp[x][i][j] = dp[x][i + 1][j];
                    } else {
                        dp[x][i][j] = dp[x][i + 1][j - 1];
                    }
                }
            }
        }
        (dp.iter().map(|d| d[0][n - 1]).sum::<i64>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::count_palindromic_subsequences("bccb".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            104860361,
            Solution::count_palindromic_subsequences(
                "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".into()
            )
        );
    }
}
