pub struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut digits = Vec::with_capacity(4);
        let mut n = num;
        while n > 0 {
            digits.push(n % 10);
            n /= 10;
        }
        digits
            .iter_mut()
            .rev()
            .skip_while(|e| **e == 9)
            .take(1)
            .for_each(|e| *e = 9);
        digits.into_iter().rev().fold(0, |n, d| n * 10 + d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(9969, Solution::maximum69_number(9669));
    }

    #[test]
    fn case2() {
        assert_eq!(9999, Solution::maximum69_number(9996));
    }

    #[test]
    fn case3() {
        assert_eq!(9999, Solution::maximum69_number(9999));
    }
}
