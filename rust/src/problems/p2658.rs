pub struct Solution;

impl Solution {
    pub fn find_max_fish(mut grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut result = 0;
        let mut q = Vec::with_capacity(m * n);
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }
                q.push((i as i32, j as i32));
                let mut count = grid[i][j];
                grid[i][j] = 0;
                while let Some((i, j)) = q.pop() {
                    for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                        if i1 < 0
                            || j1 < 0
                            || i1 == m as i32
                            || j1 == n as i32
                            || grid[i1 as usize][j1 as usize] == 0
                        {
                            continue;
                        }
                        count += grid[i1 as usize][j1 as usize];
                        q.push((i1, j1));
                        grid[i1 as usize][j1 as usize] = 0;
                    }
                }
                result = result.max(count);
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
        let grid = [[0, 2, 1, 0], [4, 0, 0, 3], [1, 0, 0, 4], [0, 3, 2, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(7, Solution::find_max_fish(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::find_max_fish(grid));
    }
}
