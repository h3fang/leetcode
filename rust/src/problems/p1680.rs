pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn pow(mut x: i64, mut n: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n >>= 1;
    }
    ans
}

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let (n, mut ans) = (n as i64, 0i64);
        let mut l = 1;
        while l <= n {
            let q = l * 2;
            let r = (q - 1).min(n);
            let m = r - l + 1;
            let q_m = pow(q, m);
            let q1_inv = pow(q - 1, MOD - 2);
            let a = (r * (q_m - 1) % MOD) * q1_inv;
            let b = (((q - m * q_m + ((m - 1) * q_m) % MOD * q) % MOD * q1_inv) % MOD) * q1_inv;
            ans = (ans * q_m + a - b).rem_euclid(MOD);
            l *= 2;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::concatenated_binary(1));
    }

    #[test]
    fn case2() {
        assert_eq!(27, Solution::concatenated_binary(3));
    }

    #[test]
    fn case3() {
        assert_eq!(505379714, Solution::concatenated_binary(12));
    }
}
