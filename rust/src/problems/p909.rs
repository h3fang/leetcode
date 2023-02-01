pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let n2 = n * n;
        let mut visited = vec![false; n2 + 1];
        let mut q = VecDeque::new();
        q.push_back((1, 0));
        while let Some((i, steps)) = q.pop_front() {
            for d in 1..=6 {
                let mut j = i + d;
                if j > n2 {
                    break;
                }
                let mut r = (j - 1) / n;
                let mut c = (j - 1) % n;
                if r % 2 == 1 {
                    c = n - 1 - c;
                }
                r = n - 1 - r;
                if board[r][c] > 0 {
                    j = board[r][c] as usize;
                }
                if j == n2 {
                    return steps + 1;
                }
                if !visited[j] {
                    visited[j] = true;
                    q.push_back((j, steps + 1));
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
        let board = [
            [-1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, 35, -1, -1, 13, -1],
            [-1, -1, -1, -1, -1, -1],
            [-1, 15, -1, -1, -1, -1],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(4, Solution::snakes_and_ladders(board));
    }

    #[test]
    fn case2() {
        let board = [[-1, -1], [-1, 3]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::snakes_and_ladders(board));
    }
}
