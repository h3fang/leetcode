pub struct Solution;

impl Solution {
    pub fn largest1_bordered_square(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut left = vec![vec![0; n]; m];
        let mut top = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                left[i][j] = if j == 0 {
                    grid[i][j]
                } else if grid[i][j] == 1 {
                    left[i][j - 1] + 1
                } else {
                    0
                };
                top[i][j] = if i == 0 {
                    grid[i][j]
                } else if grid[i][j] == 1 {
                    top[i - 1][j] + 1
                } else {
                    0
                };
            }
        }

        let mut max = 0;
        for i in 0..m {
            for j in 0..n {
                let mut lt = left[i][j].min(top[i][j]);
                while lt > 0
                    && (left[i + 1 - lt as usize][j] < lt || top[i][j + 1 - lt as usize] < lt)
                {
                    lt -= 1;
                }
                max = max.max(lt);
            }
        }
        max * max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(9, Solution::largest1_bordered_square(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 1, 0, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::largest1_bordered_square(grid));
    }
}
