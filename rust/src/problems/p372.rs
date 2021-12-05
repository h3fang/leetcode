const MOD: i64 = 1337;

pub struct Solution;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        fn pow(a: i32, mut b: i32) -> i64 {
            let mut result = 1i64;
            let mut x = a as i64;
            while b > 0 {
                if b % 2 == 1 {
                    result = (result * x) % MOD;
                }
                x = (x * x) % MOD;
                b /= 2;
            }
            result
        }

        let mut result = 1;
        for n in b {
            result = ((pow(result, 10) * pow(a, n)) % MOD) as i32;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::super_pow(2, vec![3]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::super_pow(1, vec![4, 3, 3, 8, 5, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(1198, Solution::super_pow(2147483647, vec![2, 0, 0]));
    }
}
