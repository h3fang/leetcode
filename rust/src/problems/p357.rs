pub struct Solution;

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        let mut permu = 1;
        let mut result = 1;
        for i in 1..=n {
            result += 9 * permu;
            permu *= 10 - i;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(91, Solution::count_numbers_with_unique_digits(2));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::count_numbers_with_unique_digits(0));
    }

    #[test]
    fn case3() {
        assert_eq!(739, Solution::count_numbers_with_unique_digits(3));
    }
}
