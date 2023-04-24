pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let t = target.as_bytes();
        let m = t.len();
        let n = words[0].len();
        let mut count = vec![vec![0; 26]; n];
        for w in &words {
            for (j, b) in w.as_bytes().iter().enumerate() {
                count[j][(b - b'a') as usize] += 1;
            }
        }
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[0][0] = 1;
        for i in 0..=m {
            for (j, c) in count.iter().enumerate() {
                if i < m {
                    let a = (dp[i + 1][j + 1] as i64
                        + (dp[i][j] as i64 * c[(t[i] - b'a') as usize] as i64) % MOD)
                        as i32;
                    dp[i + 1][j + 1] = a;
                }
                let a = (dp[i][j + 1] as i64 + dp[i][j] as i64) % MOD;
                dp[i][j + 1] = a as i32;
            }
        }
        dp[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["acca", "bbbb", "caca"]
            .iter()
            .map(|w| w.to_string())
            .collect();
        let target = "aba".to_string();
        assert_eq!(6, Solution::num_ways(words, target));
    }

    #[test]
    fn case2() {
        let words = ["abba", "baab"].iter().map(|w| w.to_string()).collect();
        let target = "bab".to_string();
        assert_eq!(4, Solution::num_ways(words, target));
    }
}
