pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        (0..9).all(|i| {
            let mut mask = 0;
            for &c in &board[i] {
                if c == '.' {
                    continue;
                }
                let k = c as u8 - b'0';
                if mask & (1 << k) > 0 {
                    return false;
                }
                mask |= 1 << k;
            }
            true
        }) && (0..9).all(|j| {
            let mut mask = 0;
            for r in &board {
                let c = r[j];
                if c == '.' {
                    continue;
                }
                let k = c as u8 - b'0';
                if mask & (1 << k) > 0 {
                    return false;
                }
                mask |= 1 << k;
            }
            true
        }) && [0, 3, 6].iter().all(|&i| {
            [0, 3, 6].iter().all(|&j| {
                let mut mask = 0;
                for r in &board[i..i + 3] {
                    for &c in &r[j..j + 3] {
                        if c == '.' {
                            continue;
                        }
                        let k = c as u8 - b'0';
                        if mask & (1 << k) > 0 {
                            return false;
                        }
                        mask |= 1 << k;
                    }
                }
                true
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let board = [
            ["5", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(true, Solution::is_valid_sudoku(board));
    }

    #[test]
    fn case2() {
        let board = [
            ["8", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(false, Solution::is_valid_sudoku(board));
    }
}
