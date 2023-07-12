pub struct Solution;

impl Solution {
    pub fn alternate_digit_sum(mut n: i32) -> i32 {
        let mut sum = 0;
        let mut sign = 1;
        while n > 0 {
            sum += sign * (n % 10);
            sign = -sign;
            n /= 10;
        }
        if sign == 1 {
            -sum
        } else {
            sum
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::alternate_digit_sum(521));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::alternate_digit_sum(111));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::alternate_digit_sum(886996));
    }
}
