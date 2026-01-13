pub struct Solution;

fn check(squares: &[Vec<i32>], y: f64, mut area: f64) -> bool {
    for s in squares {
        area -= s[2] as f64 * (y - s[1] as f64).min(s[2] as f64).max(0.0);
        if area <= 0.0 {
            return true;
        }
    }
    false
}

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let half = squares
            .iter()
            .map(|s| s[2] as i64 * s[2] as i64)
            .sum::<i64>() as f64
            / 2.0;
        let (mut l, mut r) = (0.0, 1.0e9);
        while r - l > 1e-5 {
            let m = (l + r) / 2.0;
            if check(&squares, m, half) {
                r = m;
            } else {
                l = m;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {a:.5}, b = {b:.5}");
    }

    #[test]
    fn case1() {
        let squares = [[0, 0, 1], [2, 2, 1]].iter().map(|v| v.to_vec()).collect();
        assert_close(1.0, Solution::separate_squares(squares));
    }

    #[test]
    fn case2() {
        let squares = [[0, 0, 2], [1, 1, 1]].iter().map(|v| v.to_vec()).collect();
        assert_close(1.16667, Solution::separate_squares(squares));
    }
}
