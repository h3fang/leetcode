pub struct Solution;

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut dp = mat.clone();
        dp.iter_mut().flatten().for_each(|e| {
            if *e != 0 {
                *e = i32::MAX - 20000;
            }
        });

        let m = mat.len();
        let n = mat[0].len();

        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
                }
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i < m - 1 {
                    dp[i][j] = dp[i][j].min(dp[i + 1][j] + 1);
                }
                if j < n - 1 {
                    dp[i][j] = dp[i][j].min(dp[i][j + 1] + 1);
                }
            }
        }

        dp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let mat = mat.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        let expected = [[0, 0, 0], [0, 1, 0], [0, 0, 0]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::update_matrix(mat));
    }

    #[test]
    fn case2() {
        let mat = [[0, 0, 0], [0, 1, 0], [1, 1, 1]];
        let mat = mat.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        let expected = [[0, 0, 0], [0, 1, 0], [1, 2, 1]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::update_matrix(mat));
    }

    #[test]
    fn case3() {
        let mat = [
            [1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            [0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            [0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
            [0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            [0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
            [1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
            [1, 1, 1, 1, 0, 1, 0, 0, 1, 1],
        ];
        let mat = mat.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        let expected = [
            [1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
            [0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
            [0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
            [1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
            [0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
            [0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
            [1, 0, 0, 0, 1, 2, 1, 1, 0, 1],
            [2, 1, 1, 1, 1, 2, 1, 0, 1, 0],
            [3, 2, 2, 1, 0, 1, 0, 0, 1, 1],
        ];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(expected, Solution::update_matrix(mat));
    }
}
