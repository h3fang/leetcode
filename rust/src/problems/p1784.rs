pub struct Solution;

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        !s.contains("01")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::check_ones_segment("1001".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::check_ones_segment("110".to_string()));
    }
}
