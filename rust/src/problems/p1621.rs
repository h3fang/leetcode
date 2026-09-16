pub struct Solution;

use std::sync::LazyLock;

const MOD: i64 = 10_0000_0007;
const M: usize = 2000;

static F: LazyLock<[i64; M]> = LazyLock::new(|| {
    let mut f = [0; M];
    f[0] = 1;
    for i in 1..f.len() {
        f[i] = (f[i - 1] * i as i64) % MOD;
    }
    f
});

static INV_F: LazyLock<[i64; M]> = LazyLock::new(|| {
    let mut inv_f = [0; M];
    inv_f[M - 1] = qpow(F[M - 1], MOD - 2);
    for i in (1..inv_f.len()).rev() {
        inv_f[i - 1] = (inv_f[i] * i as i64) % MOD;
    }
    inv_f
});

fn qpow(mut x: i64, mut n: i64) -> i64 {
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans = (ans * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    ans
}

fn comb(n: i32, k: i32) -> i32 {
    let a = (F[n as usize] * INV_F[k as usize]) % MOD;
    let b = (a * INV_F[(n - k) as usize]) % MOD;
    b as i32
}

impl Solution {
    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        comb(n + k - 1, k * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::number_of_sets(4, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::number_of_sets(3, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(796297179, Solution::number_of_sets(30, 7));
    }

    #[test]
    fn case4() {
        assert_eq!(7, Solution::number_of_sets(5, 3));
    }

    #[test]
    fn case5() {
        assert_eq!(1, Solution::number_of_sets(3, 2));
    }
}
