pub struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n > 5000 {
            return 1.0;
        }
        let n = ((n + 24) / 25) as usize;
        let mut dp = vec![vec![0.0; n + 1]; n + 1];
        dp[0].iter_mut().for_each(|e| *e = 1.0);
        dp[0][0] = 0.5;
        for i in 1..=n {
            for j in 1..=n {
                dp[i][j] = 0.25
                    * (dp[(i as i32 - 4).max(0) as usize][j]
                        + dp[(i as i32 - 3).max(0) as usize][(j as i32 - 1).max(0) as usize]
                        + dp[(i as i32 - 2).max(0) as usize][(j as i32 - 2).max(0) as usize]
                        + dp[(i as i32 - 1).max(0) as usize][(j as i32 - 3).max(0) as usize]);
            }
        }
        dp[n][n]
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
        assert_close(0.625, Solution::soup_servings(50));
    }

    #[test]
    fn case2() {
        assert_close(0.71875, Solution::soup_servings(100));
    }
}
