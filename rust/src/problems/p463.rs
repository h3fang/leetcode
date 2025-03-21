pub struct Solution;

impl Solution {
    pub fn island_perimeter(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(g: &mut [Vec<i32>], i: i32, j: i32) -> i32 {
            if i < 0
                || j < 0
                || i == g.len() as i32
                || j == g[0].len() as i32
                || g[i as usize][j as usize] == 0
            {
                return 1;
            }
            if g[i as usize][j as usize] == -1 {
                return 0;
            }
            g[i as usize][j as usize] = -1;
            let mut ans = 0;
            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                ans += dfs(g, i1, j1);
            }
            ans
        }
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    return dfs(&mut grid, i as i32, j as i32);
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
