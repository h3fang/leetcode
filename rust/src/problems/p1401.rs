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
        let x_min = if (x1..=x2).contains(&x_center) {
            0
        } else {
            (x1 - x_center).abs().min((x2 - x_center).abs())
        };
        let y_min = if (y1..=y2).contains(&y_center) {
            0
        } else {
            (y1 - y_center).abs().min((y2 - y_center).abs())
        };
        x_min * x_min + y_min * y_min <= radius * radius
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
