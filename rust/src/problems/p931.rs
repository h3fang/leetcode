pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut dp = vec![vec![0; n]; m];
        dp[m - 1].clone_from(&matrix[m - 1]);
        for i in (0..m - 1).rev() {
            for j in 0..n {
                dp[i][j] = dp[i + 1][j];
                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j - 1]);
                }
                if j + 1 < n {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j + 1]);
                }
                dp[i][j] += matrix[i][j];
            }
        }
        *dp[0].iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[2, 1, 3], [6, 5, 4], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(13, Solution::min_falling_path_sum(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[-19, 57], [-40, -5]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(-59, Solution::min_falling_path_sum(matrix));
    }
}
