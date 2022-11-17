pub struct Solution;

impl Solution {
    #[allow(clippy::too_many_arguments)]
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let rect = |x1: i32, y1: i32, x2: i32, y2: i32| (x2 - x1).max(0) * (y2 - y1).max(0);
        let r = rect(ax1, ay1, ax2, ay2) + rect(bx1, by1, bx2, by2);
        let x1 = ax1.max(bx1);
        let x2 = ax2.min(bx2);
        let y1 = ay1.max(by1);
        let y2 = ay2.min(by2);
        r - rect(x1, y1, x2, y2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(45, Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2));
    }
}
