pub struct Solution;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        fn gcd(x: i32, y: i32) -> i32 {
            if x == 0 {
                y
            } else {
                gcd(y % x, x)
            }
        }

        let g = gcd(a, b);
        let a = (a / g) as i64;
        let b = (b / g) as i64;
        let m = a + b - 1;
        let q = n as i64 / m;
        let r = n as i64 % m;

        let mut left = 0;
        let mut right = a * b;
        while left < right {
            let mid = (left + right) / 2;
            if mid / a + mid / b < r {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        (((left + q * a * b) * g as i64) % 10_0000_0007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::nth_magical_number(1, 2, 3));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::nth_magical_number(5, 2, 4));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::nth_magical_number(3, 6, 4));
    }
}
