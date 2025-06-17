pub struct Solution;

use std::sync::OnceLock;

const MOD: i64 = 10_0000_0007;

static FS: OnceLock<(Vec<i64>, Vec<i64>)> = OnceLock::new();

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

fn init() -> (Vec<i64>, Vec<i64>) {
    const N: usize = 10_0000;
    let mut f = vec![0; N];
    let mut f_inv = vec![0; N];
    f[0] = 1;
    for i in 1..N {
        f[i] = (f[i - 1] * i as i64) % MOD;
    }
    f_inv[N - 1] = pow(f[N - 1], MOD - 2);
    for i in (1..N).rev() {
        f_inv[i - 1] = (f_inv[i] * i as i64) % MOD;
    }
    (f, f_inv)
}

fn comb(n: usize, k: usize) -> i64 {
    let (f, f_inv) = FS.get_or_init(init);
    ((f[n] * f_inv[k]) % MOD * f_inv[n - k]) % MOD
}

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        let ans = comb(n as usize - 1, k as usize);
        (((ans * m as i64) % MOD) * pow(m as i64 - 1, (n - k - 1) as i64) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::count_good_arrays(3, 2, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::count_good_arrays(4, 2, 2));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::count_good_arrays(5, 2, 0));
    }
}
