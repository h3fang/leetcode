pub struct Solution;

impl Solution {
    pub fn possible_to_stamp(grid: Vec<Vec<i32>>, stamp_height: i32, stamp_width: i32) -> bool {
        let (sw, sh) = (stamp_width as usize, stamp_height as usize);
        let (m, n) = (grid.len(), grid[0].len());
        let mut sum = vec![vec![0; n + 1]; m + 1];
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                sum[i + 1][j + 1] = sum[i][j + 1] + sum[i + 1][j] - sum[i][j] + c;
            }
        }
        let mut diff = vec![vec![0; n + 2]; m + 2];
        for x in sh..=m {
            for y in sw..=n {
                let (i, j) = (x + 1 - sh, y + 1 - sw);
                if sum[x][y] - sum[x][j - 1] - sum[i - 1][y] + sum[i - 1][j - 1] == 0 {
                    diff[i][j] += 1;
                    diff[i][y + 1] -= 1;
                    diff[x + 1][j] -= 1;
                    diff[x + 1][y + 1] += 1;
                }
            }
        }
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                diff[i + 1][j + 1] += diff[i][j + 1] + diff[i + 1][j] - diff[i][j];
                if c == 0 && diff[i + 1][j + 1] == 0 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [1, 0, 0, 0],
            [1, 0, 0, 0],
            [1, 0, 0, 0],
            [1, 0, 0, 0],
            [1, 0, 0, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert!(Solution::possible_to_stamp(grid, 4, 3));
    }

    #[test]
    fn case2() {
        let grid = [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(!Solution::possible_to_stamp(grid, 2, 2));
    }
}
