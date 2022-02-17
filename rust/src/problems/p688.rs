pub struct Solution;

impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut dp = vec![vec![vec![0f64; n as usize + 1]; n as usize + 1]; k as usize + 1];
        for step in 0..=k as usize {
            for r in 0..n as usize {
                for c in 0..n as usize {
                    if step == 0 {
                        dp[step][r][c] = 1.0;
                    } else {
                        for (dr, dc) in [
                            (-2, -1),
                            (-2, 1),
                            (2, -1),
                            (2, 1),
                            (-1, -2),
                            (-1, 2),
                            (1, -2),
                            (1, 2),
                        ] {
                            let nr = r as i32 + dr;
                            let nc = c as i32 + dc;
                            if nr >= 0 && nc >= 0 && nr < n && nc < n {
                                dp[step][r][c] += dp[step - 1][nr as usize][nc as usize] / 8.0;
                            }
                        }
                    }
                }
            }
        }
        dp[k as usize][row as usize][column as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_delta(f1: f64, f2: f64, delta: f64) {
        assert!((f1 - f2).abs() < delta);
    }

    #[test]
    fn case1() {
        assert_delta(0.0625, Solution::knight_probability(3, 2, 0, 0), 1e-5);
    }

    #[test]
    fn case2() {
        assert_delta(1.0, Solution::knight_probability(1, 0, 0, 0), 1e-5);
    }

    #[test]
    fn case3() {
        assert_delta(0.00019, Solution::knight_probability(8, 30, 6, 4), 1e-5);
    }
}
