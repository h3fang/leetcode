pub struct Solution;

impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut n = num;
        let mut result = 0;
        while n > 0 {
            let d = n % 10;
            result += i32::from(num % d == 0);
            n /= 10;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::count_digits(7));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::count_digits(121));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::count_digits(1248));
    }
}
