pub struct Solution;

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        m as i64 * n as i64 / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::flower_game(3, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::flower_game(1, 1));
    }
}
