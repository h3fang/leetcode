pub struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        n > 0 && 1162261467 % n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::is_power_of_three(27));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::is_power_of_three(-1));
    }
}
