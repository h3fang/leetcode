pub struct Solution;

impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let sign = num.signum();
        let mut num = num.abs();
        let mut digits = Vec::with_capacity(32);
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        digits.sort_unstable();
        if sign == 1 {
            let first = digits.iter().position(|d| *d > 0).unwrap();
            let mut result = digits[first];
            for &d in &digits[..first] {
                result = result * 10 + d;
            }
            for &d in &digits[first + 1..] {
                result = result * 10 + d;
            }
            result
        } else {
            let mut result = 0;
            for d in digits.into_iter().rev() {
                result = result * 10 + d;
            }
            -result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(103, Solution::smallest_number(310));
    }

    #[test]
    fn case2() {
        assert_eq!(-7650, Solution::smallest_number(-7605));
    }
}
