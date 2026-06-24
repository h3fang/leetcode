pub struct Solution;

const MOD: i64 = 10_0000_0007;

fn mul(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let (m, n) = (a.len(), b[0].len());
    let mut c = vec![vec![0; n]; m];
    for (i, a) in a.iter().enumerate() {
        for j in 0..n {
            let mut sum = 0;
            for (k, a) in a.iter().enumerate() {
                sum = (sum + a * b[k][j]) % MOD;
            }
            c[i][j] = sum;
        }
    }
    c
}

fn qpow(mut a: Vec<Vec<i64>>, mut n: i32, mut ans: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    while n > 0 {
        if n & 1 == 1 {
            ans = mul(&a, &ans);
        }
        a = mul(&a, &a);
        n /= 2;
    }

    ans
}

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        let k = (r - l + 1) as usize;
        let mut m = vec![vec![0; k]; k];
        for (i, r) in m.iter_mut().enumerate() {
            for e in r.iter_mut().take(k - 1 - i) {
                *e = 1;
            }
        }

        let f = vec![vec![1]; k];
        let f = qpow(m, n - 1, f);

        ((f.iter().map(|r| r[0]).sum::<i64>() * 2) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::zig_zag_arrays(3, 4, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::zig_zag_arrays(3, 1, 3));
    }
}
