pub struct Solution;

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1.0;
        }
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;
        let mut f = vec![0.0; k + max_pts];
        for e in &mut f[k..(n + 1).min(k + max_pts)] {
            *e = 1.0;
        }
        f[k - 1] = (n - k + 1).min(max_pts) as f64 / max_pts as f64;
        for i in (0..k - 1).rev() {
            f[i] = f[i + 1] - (f[i + max_pts + 1] - f[i + 1]) / max_pts as f64;
        }
        f[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {:.5}, b = {:.5}", a, b);
    }

    #[test]
    fn case1() {
        assert_close(1.0, Solution::new21_game(10, 1, 10));
    }

    #[test]
    fn case2() {
        assert_close(0.6, Solution::new21_game(6, 1, 10));
    }

    #[test]
    fn case3() {
        assert_close(0.73278, Solution::new21_game(21, 17, 10));
    }
}
