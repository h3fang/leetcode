pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn nearest_exit(mut maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let m = maze.len() as i32;
        let n = maze[0].len() as i32;
        let mut q = VecDeque::new();
        let start = (entrance[0], entrance[1]);
        q.push_back((start, 0));
        maze[start.0 as usize][start.1 as usize] = '+';
        while let Some(((i, j), dist)) = q.pop_front() {
            if (i == 0 || j == 0 || i == m - 1 || j == n - 1) && (i, j) != start {
                return dist;
            }
            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if i1 < 0 || j1 < 0 || i1 == m || j1 == n || maze[i1 as usize][j1 as usize] == '+' {
                    continue;
                }
                q.push_back(((i1, j1), dist + 1));
                maze[i1 as usize][j1 as usize] = '+';
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
        let maze = [
            ["+", "+", ".", "+"],
            [".", ".", ".", "+"],
            ["+", "+", "+", "."],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        let entrance = vec![1, 2];
        assert_eq!(1, Solution::nearest_exit(maze, entrance));
    }

    #[test]
    fn case2() {
        let maze = [["+", "+", "+"], [".", ".", "."], ["+", "+", "+"]]
            .iter()
            .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
            .collect();
        let entrance = vec![1, 0];
        assert_eq!(2, Solution::nearest_exit(maze, entrance));
    }

    #[test]
    fn case3() {
        let maze = [[".", "+"]]
            .iter()
            .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
            .collect();
        let entrance = vec![0, 0];
        assert_eq!(-1, Solution::nearest_exit(maze, entrance));
    }
}
