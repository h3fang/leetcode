pub struct Solution;

impl Solution {
    pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid.len();

        let mut col_sum = vec![vec![0; n + 1]; n];
        for (j, col) in col_sum.iter_mut().enumerate() {
            for i in 0..n {
                col[i + 1] = col[i] + grid[i][j] as i64;
            }
        }

        let mut f = vec![vec![[0i64; 2]; n + 1]; n];
        for j in 0..n - 1 {
            let mut pre_max = f[j][0][1] - col_sum[j][0];
            for pre in 1..=n {
                f[j + 1][pre][0] = f[j][pre][0].max(pre_max + col_sum[j][pre]);
                f[j + 1][pre][1] = f[j + 1][pre][0];
                pre_max = pre_max.max(f[j][pre][1] - col_sum[j][pre]);
            }

            let mut suf_max = f[j][n][0] + col_sum[j + 1][n];
            for pre in (1..n).rev() {
                f[j + 1][pre][0] = f[j + 1][pre][0].max(suf_max - col_sum[j + 1][pre]);
                suf_max = suf_max.max(f[j][pre][0] + col_sum[j + 1][pre]);
            }

            f[j + 1][0][0] = suf_max;
            f[j + 1][0][1] = f[j][0][0].max(f[j][n][0]);
        }

        f[n - 1].iter().map(|r| r[0]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [0, 0, 0, 0, 0],
            [0, 0, 3, 0, 0],
            [0, 1, 0, 0, 0],
            [5, 0, 0, 3, 0],
            [0, 0, 0, 0, 2],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(11, Solution::maximum_score(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            [10, 9, 0, 0, 15],
            [7, 1, 0, 8, 0],
            [5, 20, 0, 11, 0],
            [0, 0, 0, 1, 2],
            [8, 12, 1, 10, 3],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(94, Solution::maximum_score(grid));
    }
}
