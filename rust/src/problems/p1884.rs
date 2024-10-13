pub struct Solution;

impl Solution {
    pub fn two_egg_drop(n: i32) -> i32 {
        (0.5 * (((8 * n + 1) as f64).sqrt() - 1.0)).ceil() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::two_egg_drop(2));
    }

    #[test]
    fn case2() {
        assert_eq!(14, Solution::two_egg_drop(100));
    }
}
