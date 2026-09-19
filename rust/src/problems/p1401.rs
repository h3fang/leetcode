pub struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let x = x1.max(x2.min(x_center));
        let y = y1.max(y2.min(y_center));
        (x - x_center) * (x - x_center) + (y - y_center) * (y - y_center) <= radius * radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_overlap(1, 0, 0, 1, -1, 3, 1));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_overlap(1, 1, 1, 1, -3, 2, -1));
    }

    #[test]
    fn case3() {
        assert!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1));
    }
}
