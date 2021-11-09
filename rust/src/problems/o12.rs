pub struct Solution;

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, w: &[u8]) -> bool {
            if w.is_empty() {
                true
            } else if i != usize::MAX
                && i < board.len()
                && j != usize::MAX
                && j < board[0].len()
                && board[i][j] == w[0] as char
            {
                board[i][j] = '#';
                let r = dfs(board, i - 1, j, &w[1..])
                    || dfs(board, i + 1, j, &w[1..])
                    || dfs(board, i, j - 1, &w[1..])
                    || dfs(board, i, j + 1, &w[1..]);
                board[i][j] = w[0] as char;
                r
            } else {
                false
            }
        }

        let w = word.as_bytes();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if dfs(&mut board, i, j, w) {
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
            ["A", "B", "C", "E"],
            ["S", "F", "C", "S"],
            ["A", "D", "E", "E"],
        ];
        let board = board
            .into_iter()
            .map(|r| {
                r.iter()
                    .map(|e| e.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let word = "ABCCED".to_string();
        assert_eq!(true, Solution::exist(board, word));
    }

    #[test]
    fn case2() {
        let board = [["a", "b"], ["c", "d"]];
        let board = board
            .into_iter()
            .map(|r| {
                r.iter()
                    .map(|e| e.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let word = "abcd".to_string();
        assert_eq!(false, Solution::exist(board, word));
    }
}
