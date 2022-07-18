pub struct Solution;

const MODULUS: u32 = 1000000007;

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let mut dp = vec![vec![0u32; k as usize + 1]; 2];
        dp[0][0] = 1;
        let mut c = 1;
        for i in 1..=n as usize {
            for j in 0..=k as usize {
                dp[c][j] = dp[1 - c][j] + if j > 0 { dp[c][j - 1] } else { 0 } + MODULUS
                    - if j >= i { dp[1 - c][j - i] } else { 0 };
                dp[c][j] %= MODULUS;
            }
            c = 1 - c;
        }

        dp[1 - c][k as usize] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::k_inverse_pairs(3, 0));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::k_inverse_pairs(3, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(21670, Solution::k_inverse_pairs(10, 10));
    }
}
