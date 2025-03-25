pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let n = n as usize;
        let mut f = [1; 10];
        for _ in 1..n {
            f = [
                (f[4] + f[6]) % MOD,
                (f[8] + f[6]) % MOD,
                (f[7] + f[9]) % MOD,
                (f[4] + f[8]) % MOD,
                (f[3] + f[9] + f[0]) % MOD,
                0,
                (f[1] + f[7] + f[0]) % MOD,
                (f[2] + f[6]) % MOD,
                (f[1] + f[3]) % MOD,
                (f[2] + f[4]) % MOD,
            ];
        }
        (f.into_iter().sum::<i64>() % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::knight_dialer(1));
    }

    #[test]
    fn case2() {
        assert_eq!(20, Solution::knight_dialer(2));
    }

    #[test]
    fn case3() {
        assert_eq!(136006598, Solution::knight_dialer(3131));
    }
}
