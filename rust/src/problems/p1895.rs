pub struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut row = vec![vec![0; n + 1]; m];
        let mut col = vec![vec![0; n]; m + 1];
        let mut diag = vec![vec![0; n + 1]; m + 1];
        let mut diag_rev = vec![vec![0; n + 1]; m + 1];

        for (i, r) in grid.iter().enumerate() {
            for (j, &x) in r.iter().enumerate() {
                row[i][j + 1] = row[i][j] + x;
                col[i + 1][j] = col[i][j] + x;
                diag[i + 1][j + 1] = diag[i][j] + x;
                diag_rev[i + 1][j] = diag_rev[i][j + 1] + x;
            }
        }

        for k in (1..=m.min(n)).rev() {
            for i in k..m + 1 {
                for j in k..n + 1 {
                    let d = diag[i][j] - diag[i - k][j - k];
                    if diag_rev[i][j - k] - diag_rev[i - k][j] == d
                        && (i - k..i).all(|r| row[r][j] - row[r][j - k] == d)
                        && (j - k..j).all(|c| col[i][c] - col[i - k][c] == d)
                    {
                        return k as i32;
                    }
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [7, 1, 4, 5, 6],
            [2, 5, 1, 6, 4],
            [1, 5, 4, 3, 2],
            [1, 2, 7, 3, 4],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(3, Solution::largest_magic_square(grid));
    }

    #[test]
    fn case2() {
        let grid = [[5, 1, 3, 1], [9, 3, 3, 1], [1, 3, 3, 8]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::largest_magic_square(grid));
    }
}
