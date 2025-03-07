pub struct Solution;

impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let (m, n) = (board.len() as i32, board[0].len() as i32);
        for (di, dj) in [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ] {
            let (mut i, mut j) = (r_move + di, c_move + dj);
            if i < 0 || j < 0 || i >= m || j >= n {
                continue;
            }
            let c = board[i as usize][j as usize];
            if c == color || c == '.' {
                continue;
            }
            loop {
                i += di;
                j += dj;
                if i < 0 || j < 0 || i >= m || j >= n || board[i as usize][j as usize] == '.' {
                    break;
                }
                if board[i as usize][j as usize] == color {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let board = [
            [".", ".", ".", "B", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
            ["W", "B", "B", ".", "W", "W", "W", "B"],
            [".", ".", ".", "B", ".", ".", ".", "."],
            [".", ".", ".", "B", ".", ".", ".", "."],
            [".", ".", ".", "W", ".", ".", ".", "."],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert!(Solution::check_move(board, 4, 3, 'B'));
    }

    #[test]
    fn case2() {
        let board = [
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", "B", ".", ".", "W", ".", ".", "."],
            [".", ".", "W", ".", ".", ".", ".", "."],
            [".", ".", ".", "W", "B", ".", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", "B", "W", ".", "."],
            [".", ".", ".", ".", ".", ".", "W", "."],
            [".", ".", ".", ".", ".", ".", ".", "B"],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert!(!Solution::check_move(board, 4, 4, 'W'));
    }
}
