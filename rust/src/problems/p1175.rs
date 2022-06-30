pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        fn is_prime(n: i32) -> bool {
            if n == 1 {
                return false;
            }
            let mut x = 2;
            while x * x <= n {
                if n % x == 0 {
                    return false;
                }
                x += 1;
            }
            true
        }

        fn factorial(n: i32) -> i64 {
            let mut r = 1;
            for i in 1..=n {
                r = (r * i as i64) % MOD;
            }
            r
        }

        let p = (1..=n).filter(|&i| is_prime(i)).count() as i32;
        let np = n - p;
        let r = (factorial(p) * factorial(np)) % MOD;
        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::num_prime_arrangements(5));
    }

    #[test]
    fn case2() {
        assert_eq!(682289015, Solution::num_prime_arrangements(100));
    }
}
