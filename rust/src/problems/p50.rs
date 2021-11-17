pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn pow(x: f64, mut n: i64) -> f64 {
            if n < 0 {
                1.0 / pow(x, -n)
            } else {
                let mut r = 1.0;
                let mut factor = x;
                while n > 0 {
                    if n & 1 > 0 {
                        r *= factor;
                    }
                    factor *= factor;
                    n /= 2;
                }
                r
            }
        }
        pow(x, n as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!((1024.0 - Solution::my_pow(2.0, 10)).abs() < 1e-5)
    }

    #[test]
    fn case2() {
        assert!((9.26100 - Solution::my_pow(2.1, 3)).abs() < 1e-5)
    }

    #[test]
    fn case3() {
        assert!((0.2500 - Solution::my_pow(2.0, -2)).abs() < 1e-5)
    }

    #[test]
    fn case4() {
        assert!((1.0 - Solution::my_pow(1.0, -2147483648)).abs() < 1e-5)
    }
}
