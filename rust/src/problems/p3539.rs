pub struct Solution;

use std::sync::OnceLock;

static F: OnceLock<([i64; N], [i64; N])> = OnceLock::new();

const MOD: i64 = 10_0000_0007;
const N: usize = 31;

fn init() -> ([i64; N], [i64; N]) {
    let mut f = [0; N];
    f[0] = 1;
    for i in 1..N {
        f[i] = (f[i - 1] * i as i64) % MOD;
    }

    let mut inv = [0; 31];
    inv[N - 1] = pow(f[N - 1], MOD - 2);
    for i in (1..N).rev() {
        inv[i - 1] = (inv[i] * i as i64) % MOD;
    }
    (f, inv)
}

fn pow(mut x: i64, mut n: i64) -> i64 {
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

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        let (frac, inv) = F.get_or_init(init);
        let (m, n, k) = (m as usize, nums.len(), k as usize);

        let mut p = vec![vec![0; m + 1]; n];
        for (i, v) in p.iter_mut().enumerate() {
            v[0] = 1;
            for j in 1..=m {
                v[j] = (v[j - 1] * nums[i] as i64) % MOD;
            }
        }

        let mut f = vec![vec![vec![vec![0; k + 1]; m / 2 + 1]; m + 1]; n + 1];
        for x in 0..=m / 2 {
            let c = x.count_ones() as usize;
            if c <= k {
                f[n][0][x][c] = 1;
            }
        }

        for i in (0..n).rev() {
            for left_m in 0..=m {
                for x in 0..=m / 2 {
                    for left_k in 0..=k {
                        let mut r = 0;
                        for (j, inv) in inv[0..=left_m.min(m - x)].iter().enumerate() {
                            let bit = (x + j) & 1;
                            if bit <= left_k {
                                let c = f[i + 1][left_m - j][(x + j) / 2][left_k - bit];
                                r = (r + (c * p[i][j]) % MOD * inv) % MOD;
                            }
                        }
                        f[i][left_m][x][left_k] = r;
                    }
                }
            }
        }

        ((f[0][m][0][k] * frac[m]) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            991600007,
            Solution::magical_sum(5, 5, vec![1, 10, 100, 10000, 1000000])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(170, Solution::magical_sum(2, 2, vec![5, 4, 3, 2, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(170, Solution::magical_sum(2, 2, vec![5, 4, 3, 2, 1]));
    }
}
