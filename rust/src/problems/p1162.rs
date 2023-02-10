pub struct Solution;

impl Solution {
    pub fn max_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let max_distance = (m + n + 1) as i32;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                } else {
                    grid[i][j] = max_distance;
                    grid[i][j] = grid[i][j].min(
                        (if i > 0 {
                            grid[i - 1][j] + 1
                        } else {
                            max_distance
                        })
                        .min(if j > 0 {
                            grid[i][j - 1] + 1
                        } else {
                            max_distance
                        }),
                    );
                }
            }
        }
        let mut result = i32::MIN;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                grid[i][j] = grid[i][j].min(
                    (if i < m - 1 {
                        grid[i + 1][j] + 1
                    } else {
                        max_distance
                    })
                    .min(if j < n - 1 {
                        grid[i][j + 1] + 1
                    } else {
                        max_distance
                    }),
                );
                result = result.max(grid[i][j]);
            }
        }
        if result == 0 || result == max_distance {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 0, 1], [0, 0, 0], [1, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::max_distance(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 0, 0], [0, 0, 0], [0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::max_distance(grid));
    }
}
