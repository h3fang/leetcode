pub struct Solution;

impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        fn gcd(x: i64, y: i64) -> i64 {
            if x == 0 { y } else { gcd(y % x, x) }
        }

        let n = n as i64;
        let a = a as i64;
        let b = b as i64;
        let c = a * b / gcd(a, b);

        let mut left = 0;
        let mut right = a.min(b) * n;
        while left < right {
            let mid = (left + right) / 2;
            let count = mid / a + mid / b - mid / c;
            if count < n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        (right % 10_0000_0007) as i32
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
