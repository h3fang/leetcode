pub struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] == 'X' {
                    if i > 0 && board[i - 1][j] == 'X' {
                        continue;
                    }
                    if j > 0 && board[i][j - 1] == 'X' {
                        continue;
                    }
                    result += 1;
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
        let board = [
            ["X", ".", ".", "X"],
            [".", ".", ".", "X"],
            [".", ".", ".", "X"],
        ];
        let board = board
            .iter()
            .map(|row| row.iter().map(|c| c.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(2, Solution::count_battleships(board));
    }

    #[test]
    fn case2() {
        let board = [["."]];
        let board = board
            .iter()
            .map(|row| row.iter().map(|c| c.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(0, Solution::count_battleships(board));
    }
}
