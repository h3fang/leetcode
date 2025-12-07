pub struct Solution;

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high + 1) / 2 - low / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_odds(3, 7));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::count_odds(8, 10));
    }
}
