pub struct Solution;

impl Solution {
    pub fn count_pyramids(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut height = vec![vec![0; n]; m];
        let mut result = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                if i == 0 || j == 0 || j == n - 1 {
                    height[i][j] = 1;
                } else {
                    height[i][j] = height[i - 1][j - 1]
                        .min(height[i - 1][j + 1])
                        .min(height[i - 1][j])
                        + 1;
                    result += height[i][j] - 1;
                }
            }
        }

        for i in (0..m).rev() {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                if i == m - 1 || j == 0 || j == n - 1 {
                    height[i][j] = 1;
                } else {
                    height[i][j] = height[i + 1][j - 1]
                        .min(height[i + 1][j + 1])
                        .min(height[i + 1][j])
                        + 1;
                    result += height[i][j] - 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![vec![0, 1, 1, 0], vec![1, 1, 1, 1]];
        assert_eq!(2, Solution::count_pyramids(grid));
    }

    #[test]
    fn case2() {
        let grid = vec![vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(2, Solution::count_pyramids(grid));
    }

    #[test]
    fn case3() {
        let grid = vec![
            vec![1, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 0, 0, 1],
        ];
        assert_eq!(13, Solution::count_pyramids(grid));
    }

    #[test]
    fn case4() {
        let grid = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        assert_eq!(0, Solution::count_pyramids(grid));
    }
}
