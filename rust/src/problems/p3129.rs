pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let (zero, one, limit) = (zero as usize, one as usize, limit as usize);
        let mut f = vec![vec![[0; 2]; one + 1]; zero + 1];
        for e in f.iter_mut().skip(1).take(zero.min(limit)) {
            e[0][0] = 1;
        }
        for j in 1..one.min(limit) + 1 {
            f[0][j][1] = 1;
        }
        for i in 1..zero + 1 {
            for j in 1..one + 1 {
                f[i][j][0] = (f[i - 1][j][0]
                    + f[i - 1][j][1]
                    + if i > limit {
                        MOD - f[i - limit - 1][j][1]
                    } else {
                        0
                    })
                    % MOD;
                f[i][j][1] = (f[i][j - 1][0]
                    + f[i][j - 1][1]
                    + if j > limit {
                        MOD - f[i][j - limit - 1][0]
                    } else {
                        0
                    })
                    % MOD;
            }
        }
        ((f[zero][one][0] + f[zero][one][1]) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::number_of_stable_arrays(1, 1, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::number_of_stable_arrays(1, 2, 1));
    }

    #[test]
    fn case3() {
        assert_eq!(14, Solution::number_of_stable_arrays(3, 3, 2));
    }
}
