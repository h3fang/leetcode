pub struct Solution;

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let max = (sx - fx).abs().max((sy - fy).abs());
        !(max == 0 && t == 1) && max <= t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_reachable_at_time(2, 4, 7, 7, 6));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_reachable_at_time(3, 1, 7, 3, 3));
    }
}
