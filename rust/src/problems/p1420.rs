pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let (n, m, k) = (n as usize, m as usize, k as usize);
        let mut f = vec![vec![vec![0; m + 1]; k + 1]; n + 1];
        f[1][1] = vec![1; m + 1];
        f[1][1][0] = 0;

        for i in 2..=n {
            if let Ok([a, b]) = f.get_disjoint_mut([i, i - 1]) {
                for s in 1..=k.min(i) {
                    let mut presum = 0;
                    for j in 1..=m {
                        a[s][j] = ((b[s][j] as i64 * j as i64) % MOD) as i32;
                        a[s][j] = (a[s][j] + presum) % MOD as i32;
                        presum = (presum + b[s - 1][j]) % MOD as i32;
                    }
                }
            }
        }
        f[n][k]
            .iter()
            .skip(1)
            .fold(0, |acc, e| (acc + e) % MOD as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::num_of_arrays(2, 3, 1));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::num_of_arrays(5, 2, 3));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::num_of_arrays(9, 1, 1));
    }
}
