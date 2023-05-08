pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn min_push_box(grid: Vec<Vec<char>>) -> i32 {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;
        let mut target = (0, 0);
        let mut player = (0, 0);
        let mut cube = (0, 0);
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                let p = (i as i32, j as i32);
                match c {
                    'S' => player = p,
                    'B' => cube = p,
                    'T' => target = p,
                    _ => {}
                }
            }
        }
        let index = |i: i32, j: i32| -> i32 { i * n + j };
        let is_valid = |i: i32, j: i32| -> bool {
            i >= 0 && j >= 0 && i < m && j < n && grid[i as usize][j as usize] != '#'
        };
        let k = (m as usize) * (n as usize);
        let mut dp = vec![vec![i32::MAX; k]; k];
        let mut q = VecDeque::new();
        let ip = index(player.0, player.1);
        let ib = index(cube.0, cube.1);
        let it = index(target.0, target.1);
        dp[ip as usize][ib as usize] = 0;
        q.push_back((ip, ib));
        while !q.is_empty() {
            let mut q1 = VecDeque::new();
            while let Some((ip, ib)) = q.pop_front() {
                if ib == it {
                    return dp[ip as usize][ib as usize];
                }
                let (px, py) = (ip / n, ip % n);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (px1, py1) = (px + dx, py + dy);
                    if !is_valid(px1, py1) {
                        continue;
                    }
                    let ip1 = index(px1, py1);
                    if ib == index(px1, py1) {
                        let (bx, by) = (ib / n, ib % n);
                        let (bx1, by1) = (bx + dx, by + dy);
                        let ib1 = index(bx1, by1);
                        if is_valid(bx1, by1)
                            && dp[ip1 as usize][ib1 as usize] > dp[ip as usize][ib as usize] + 1
                        {
                            dp[ip1 as usize][ib1 as usize] = dp[ip as usize][ib as usize] + 1;
                            q1.push_back((ip1, ib1));
                        }
                    } else if dp[ip1 as usize][ib as usize] > dp[ip as usize][ib as usize] {
                        dp[ip1 as usize][ib as usize] = dp[ip as usize][ib as usize];
                        q.push_back((ip1, ib));
                    }
                }
            }
            q = q1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [
            ["#", "#", "#", "#", "#", "#"],
            ["#", "T", "#", "#", "#", "#"],
            ["#", ".", ".", "B", ".", "#"],
            ["#", ".", "#", "#", ".", "#"],
            ["#", ".", ".", ".", "S", "#"],
            ["#", "#", "#", "#", "#", "#"],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(3, Solution::min_push_box(grid));
    }

    #[test]
    fn case2() {
        let grid = [
            ["#", "#", "#", "#", "#", "#"],
            ["#", "T", "#", "#", "#", "#"],
            ["#", ".", ".", "B", ".", "#"],
            ["#", "#", "#", "#", ".", "#"],
            ["#", ".", ".", ".", "S", "#"],
            ["#", "#", "#", "#", "#", "#"],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(-1, Solution::min_push_box(grid));
    }

    #[test]
    fn case3() {
        let grid = [
            ["#", "#", "#", "#", "#", "#"],
            ["#", "T", ".", ".", "#", "#"],
            ["#", ".", "#", "B", ".", "#"],
            ["#", ".", ".", ".", ".", "#"],
            ["#", ".", ".", ".", "S", "#"],
            ["#", "#", "#", "#", "#", "#"],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(5, Solution::min_push_box(grid));
    }
}
