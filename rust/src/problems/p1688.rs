pub struct Solution;

impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::number_of_matches(7));
    }

    #[test]
    fn case2() {
        assert_eq!(13, Solution::number_of_matches(14));
    }
}
