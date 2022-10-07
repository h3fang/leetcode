pub struct Solution;

impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        let mut result = 0;
        for i in 1..=a.min(b) {
            if a % i == 0 && b % i == 0 {
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::common_factors(12, 6));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::common_factors(25, 30));
    }
}
