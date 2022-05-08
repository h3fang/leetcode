pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        fn key_symbols(k: u8) -> usize {
            match k {
                2 | 3 | 4 | 5 | 6 | 8 => 3,
                _ => 4,
            }
        }
        let keys = pressed_keys.as_bytes();
        let n = keys.len();
        let mut dp = vec![0; n + 1];
        dp[n] = 1;
        for i in (0..n).rev() {
            let d = keys[i] - b'0';
            let k = key_symbols(d);
            let mut sum = 0i64;
            for j in i..n {
                if keys[j] != keys[i] || (j - i + 1) > k {
                    break;
                }
                sum = (sum + dp[j + 1]) % MOD;
            }
            dp[i] = sum;
        }
        dp[0] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::count_texts("22233".into()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            82876089,
            Solution::count_texts("222222222222222222222222222222222222".into())
        );
    }
}
