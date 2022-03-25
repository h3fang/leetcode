pub struct Solution;

impl Solution {
    pub fn trailing_zeroes(mut n: i32) -> i32 {
        let mut result = 0;
        while n >= 5 {
            n /= 5;
            result += n;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(748, Solution::trailing_zeroes(3000));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::trailing_zeroes(5));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::trailing_zeroes(0));
    }
}
