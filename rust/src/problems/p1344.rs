pub struct Solution;

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let d = 0.5 * f64::from((hour * 60 - minutes * 11).abs());
        d.min(360.0 - d)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {:.6}, b = {:.6}", a, b)
    }

    #[test]
    fn case1() {
        assert_close(165.0, Solution::angle_clock(12, 30));
    }

    #[test]
    fn case2() {
        assert_close(75.0, Solution::angle_clock(3, 30));
    }

    #[test]
    fn case3() {
        assert_close(7.5, Solution::angle_clock(3, 15));
    }

    #[test]
    fn case4() {
        assert_close(155.0, Solution::angle_clock(4, 50));
    }

    #[test]
    fn case5() {
        assert_close(0.0, Solution::angle_clock(12, 0));
    }
}
