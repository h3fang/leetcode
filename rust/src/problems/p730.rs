pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![0; n]; n];
        for (i, d) in dp.iter_mut().enumerate() {
            d[i] = 1;
        }
        let mut prev = vec![[0; 4]; n];
        let mut next = vec![[0; 4]; n];

        let mut pos = [-1; 4];
        for (i, &b) in s.iter().enumerate() {
            prev[i] = pos;
            pos[(b - b'a') as usize] = i as i32;
        }

        pos = [n as i32; 4];
        for (i, &b) in s.iter().enumerate().rev() {
            next[i] = pos;
            pos[(b - b'a') as usize] = i as i32;
        }

        for dist in 2..=n {
            for j in dist - 1..n {
                let i = j + 1 - dist;
                if s[i] == s[j] {
                    let x = (s[i] - b'a') as usize;
                    let low = next[i][x];
                    let high = prev[j][x];
                    dp[i][j] = match low.cmp(&high) {
                        std::cmp::Ordering::Less => {
                            (dp[i + 1][j - 1] * 2 - dp[(low + 1) as usize][(high - 1) as usize]
                                + MOD)
                                % MOD
                        }
                        std::cmp::Ordering::Equal => (1 + dp[i + 1][j - 1] * 2) % MOD,
                        std::cmp::Ordering::Greater => (2 + dp[i + 1][j - 1] * 2) % MOD,
                    };
                } else {
                    dp[i][j] = (dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1] + MOD) % MOD;
                }
            }
        }
        dp[0][n - 1] as i32
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
