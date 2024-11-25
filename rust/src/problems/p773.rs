pub struct Solution;

use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let board = [
            board[0][0] as u8,
            board[0][1] as u8,
            board[0][2] as u8,
            board[1][0] as u8,
            board[1][1] as u8,
            board[1][2] as u8,
        ];
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((board, 0));
        while let Some((mut b, steps)) = q.pop_front() {
            if b == [1, 2, 3, 4, 5, 0] {
                return steps;
            }
            visited.insert(b);
            let i = b.iter().position(|&x| x == 0).unwrap();
            let (r, c) = ((i / 3) as i32, (i % 3) as i32);
            for (r1, c1) in [(r + 1, c), (r - 1, c), (r, c + 1), (r, c - 1)] {
                if (0..2).contains(&r1) && (0..3).contains(&c1) {
                    b.swap(i, (r1 * 3 + c1) as usize);
                    if !visited.contains(&b) {
                        q.push_back((b, steps + 1));
                    }
                    b.swap(i, (r1 * 3 + c1) as usize);
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
        let board = [[1, 2, 3], [4, 0, 5]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::sliding_puzzle(board));
    }

    #[test]
    fn case2() {
        let board = [[1, 2, 3], [5, 4, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(-1, Solution::sliding_puzzle(board));
    }

    #[test]
    fn case3() {
        let board = [[4, 1, 2], [5, 0, 3]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(5, Solution::sliding_puzzle(board));
    }
}
