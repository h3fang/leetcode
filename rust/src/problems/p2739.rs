pub struct Solution;

impl Solution {
    pub fn distance_traveled(m: i32, a: i32) -> i32 {
        10 * (m + ((m - 1) / 4).min(a))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(60, Solution::distance_traveled(5, 10));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::distance_traveled(1, 2));
    }
}
