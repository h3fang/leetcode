pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn flip_chess(chessboard: Vec<String>) -> i32 {
        fn check(b: &[Vec<u8>], i: usize, j: usize, di: i32, dj: i32) -> bool {
            let mut i = i as i32 + di;
            let mut j = j as i32 + dj;
            while i >= 0 && j >= 0 && i < b.len() as i32 && j < b[0].len() as i32 {
                let c = b[i as usize][j as usize];
                if c == b'X' {
                    return true;
                } else if c == b'.' {
                    return false;
                }
                i += di;
                j += dj;
            }
            false
        }

        fn flip(b: &[Vec<u8>], i: usize, j: usize) -> i32 {
            let mut b = b.to_vec();
            let mut q = VecDeque::new();
            q.push_back((i, j));
            b[i][j] = b'X';
            let mut result = 0;
            while let Some((i, j)) = q.pop_front() {
                for (di, dj) in [
                    (0, 1),
                    (0, -1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (-1, -1),
                    (-1, 1),
                    (1, -1),
                ] {
                    if check(&b, i, j, di, dj) {
                        let mut i = i as i32 + di;
                        let mut j = j as i32 + dj;
                        while b[i as usize][j as usize] == b'O' {
                            b[i as usize][j as usize] = b'X';
                            q.push_back((i as usize, j as usize));
                            i += di;
                            j += dj;
                            result += 1;
                        }
                    }
                }
            }
            result
        }

        let b = chessboard
            .iter()
            .map(|r| r.as_bytes().to_vec())
            .collect::<Vec<_>>();
        let mut result = 0;
        for (i, row) in chessboard.iter().enumerate() {
            for (j, &c) in row.as_bytes().iter().enumerate() {
                if c == b'.' {
                    result = result.max(flip(&b, i, j));
                }
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
        let chessboard = ["....X.", "....X.", "XOOO..", "......", "......"]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(3, Solution::flip_chess(chessboard));
    }

    #[test]
    fn case2() {
        let chessboard = [".X.", ".O.", "XO."]
            .iter()
            .map(|r| r.to_string())
            .collect();
        assert_eq!(2, Solution::flip_chess(chessboard));
    }

    #[test]
    fn case3() {
        let chessboard = [
            ".......", ".......", ".......", "X......", ".O.....", "..O....", "....OOX",
        ]
        .iter()
        .map(|r| r.to_string())
        .collect();
        assert_eq!(4, Solution::flip_chess(chessboard));
    }
}
