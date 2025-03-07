pub struct Solution;

impl Solution {
    pub fn island_perimeter(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(g: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
            if g[i][j] != 1 {
                return 0;
            }
            g[i][j] = -1;
            4 + if i > 0 && g[i - 1][j].abs() == 1 {
                -1 + dfs(g, i - 1, j)
            } else {
                0
            } + if i + 1 < g.len() && g[i + 1][j].abs() == 1 {
                -1 + dfs(g, i + 1, j)
            } else {
                0
            } + if j > 0 && g[i][j - 1].abs() == 1 {
                -1 + dfs(g, i, j - 1)
            } else {
                0
            } + if j + 1 < g[0].len() && g[i][j + 1].abs() == 1 {
                -1 + dfs(g, i, j + 1)
            } else {
                0
            }
        }
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    return dfs(&mut grid, i, j);
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 0, 0], [1, 1, 1, 0], [0, 1, 0, 0], [1, 1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(16, Solution::island_perimeter(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::island_perimeter(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::island_perimeter(grid));
    }
}
