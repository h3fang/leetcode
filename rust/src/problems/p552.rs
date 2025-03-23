pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mut f = [[0; 3]; 2];
        f[0][0] = 1;
        for _ in 1..=n {
            let mut f1 = [[0; 3]; 2];
            for j in 0..2 {
                for k in 0..3 {
                    f1[j][0] = (f1[j][0] + f[j][k]) % MOD;
                }
            }
            for k in 0..3 {
                f1[1][0] = (f1[1][0] + f[0][k]) % MOD;
            }
            for j in 0..2 {
                for k in 0..2 {
                    f1[j][k + 1] = (f1[j][k + 1] + f[j][k]) % MOD;
                }
            }
            f = f1;
        }
        f.iter().flatten().fold(0, |acc, x| (acc + x) % MOD)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::check_record(2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::check_record(1));
    }

    #[test]
    fn case3() {
        assert_eq!(183236316, Solution::check_record(10101));
    }
}
