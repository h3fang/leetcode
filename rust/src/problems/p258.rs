pub struct Solution;

impl Solution {
    pub fn add_digits(mut num: i32) -> i32 {
        if num < 10 {
            return num;
        }
        let mut result = 0;
        while num > 0 {
            result += num % 10;
            num /= 10;
        }
        Self::add_digits(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::add_digits(38));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::add_digits(0));
    }
}
