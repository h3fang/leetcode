pub struct Solution;

const DIR: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];

fn dfs(
    grid: &[Vec<i32>],
    i: i32,
    j: i32,
    k: usize,
    turn: u8,
    v: i32,
    cache: &mut [Vec<[i32; 4]>],
) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let (i, j) = (i + DIR[k].0, j + DIR[k].1);
    if i < 0 || j < 0 || i >= m as i32 || j >= n as i32 || grid[i as usize][j as usize] != v {
        return 0;
    }
    if turn == 0 && cache[i as usize][j as usize][k] > 0 {
        return cache[i as usize][j as usize][k];
    }
    let mut ans = dfs(grid, i, j, k, turn, 2 - v, cache) + 1;
    if turn == 1 {
        let max = [m as i32 - i, j + 1, i + 1, n as i32 - j];
        let k = (k + 1) % 4;
        if max[k].min(max[(k + 3) % 4]) > ans {
            ans = ans.max(dfs(grid, i, j, k, 0, 2 - v, cache) + 1);
        }
    } else {
        cache[i as usize][j as usize][k] = ans;
    }
    ans
}

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut cache = vec![vec![[0; 4]; n]; m];
        let mut ans = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c != 1 {
                    continue;
                }
                let max = [m - i, j + 1, i + 1, n - j];
                for (k, max) in max.into_iter().enumerate() {
                    if max as i32 <= ans {
                        continue;
                    }
                    ans = ans.max(dfs(&grid, i as i32, j as i32, k, 1, 2, &mut cache) + 1);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            [2, 2, 1, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(5, Solution::len_of_v_diagonal(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            [2, 2, 2, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(4, Solution::len_of_v_diagonal(grid));
    }

    #[test]
    fn case3() {
        let grid = [
            [1, 2, 2, 2, 2],
            [2, 2, 2, 2, 0],
            [2, 0, 0, 0, 0],
            [0, 0, 2, 2, 2],
            [2, 0, 0, 2, 0],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(5, Solution::len_of_v_diagonal(grid));
    }

    #[test]
    fn case4() {
        let grid = [[1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::len_of_v_diagonal(grid));
    }
}
