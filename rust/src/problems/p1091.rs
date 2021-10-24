use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut q = BinaryHeap::new();
        let mut closed = vec![vec![false; n]; m];
        let mut dist = vec![vec![i32::MAX; n]; m];
        if grid[0][0] == 0 {
            dist[0][0] = 1;
            q.push((-1, (0, 0)));
        }

        let heuristic =
            |x: i64, y: i64| ((x - m as i64 + 1).abs().max((y - n as i64 + 1).abs())) as i32;
        let is_valid_index = |x: i64, y: i64| x >= 0 && x < m as i64 && y >= 0 && y < n as i64;

        while let Some((_, (x, y))) = q.pop() {
            let d = dist[x][y];
            if (x, y) == (m - 1, n - 1) {
                return d;
            }

            closed[x][y] = true;

            for (dx, dy) in &[
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                let x_new = x as i64 + dx;
                let y_new = y as i64 + dy;
                if is_valid_index(x_new, y_new)
                    && grid[x_new as usize][y_new as usize] == 0
                    && d + 1 < dist[x_new as usize][y_new as usize]
                    && !closed[x_new as usize][y_new as usize]
                {
                    dist[x_new as usize][y_new as usize] = d + 1;
                    q.push((
                        -d - 1 - heuristic(x_new, y_new),
                        (x_new as usize, y_new as usize),
                    ));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(2, Solution::shortest_path_binary_matrix(grid));
    }

    #[test]
    fn case2() {
        let grid = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(4, Solution::shortest_path_binary_matrix(grid));
    }

    #[test]
    fn case3() {
        let grid = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        assert_eq!(-1, Solution::shortest_path_binary_matrix(grid));
    }
}
