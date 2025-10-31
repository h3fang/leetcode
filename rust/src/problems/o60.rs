pub struct Solution;

const ONE: f64 = 1.0 / 6.0;

impl Solution {
    pub fn dices_probability(n: i32) -> Vec<f64> {
        let n = n as usize;
        let mut dp = vec![vec![0.0; 6 * n + 1]; n];
        dp[0][1..=6].iter_mut().for_each(|e| *e = ONE);
        for i in 1..n {
            for j in i + 1..=6 * (i + 1) {
                for k in 1..=6 {
                    if j <= k {
                        break;
                    }
                    dp[i][j] += ONE * dp[i - 1][j - k];
                }
            }
        }

        dp[n - 1][n..=6 * n].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn all_close(expected: Vec<f64>, result: Vec<f64>) {
        assert_eq!(expected.len(), result.len());
        for (a, b) in expected.iter().zip(&result) {
            assert!((a - b).abs() < 1e-5);
        }
    }

    #[test]
    fn case1() {
        all_close(vec![1.0 / 6.0; 6], Solution::dices_probability(1));
    }

    #[test]
    fn case2() {
        all_close(
            vec![
                0.02778, 0.05556, 0.08333, 0.11111, 0.13889, 0.16667, 0.13889, 0.11111, 0.08333,
                0.05556, 0.02778,
            ],
            Solution::dices_probability(2),
        );
    }
}
