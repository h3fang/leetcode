pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn bt(board: &mut Vec<Vec<char>>, word: &[char], i: usize, j: usize) -> bool {
            if word.is_empty() {
                return true;
            }
            let height = board.len();
            let width = board[0].len();
            if board[i][j] == word[0] {
                board[i][j] = '#';
                let r = bt(board, &word[1..], i.saturating_sub(1), j)
                    || bt(board, &word[1..], (i + 1).min(height - 1), j)
                    || bt(board, &word[1..], i, j.saturating_sub(1))
                    || bt(board, &word[1..], i, (j + 1).min(width - 1));
                board[i][j] = word[0];
                r
            } else {
                false
            }
        }

        let mut board = board;
        let word = word.chars().collect::<Vec<_>>();
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if bt(&mut board, &word, i, j) {
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
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let expected = true;
        assert_eq!(expected, Solution::exist(board, word));
    }

    #[test]
    fn case2() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "SEE".to_string();
        let expected = true;
        assert_eq!(expected, Solution::exist(board, word));
    }

    #[test]
    fn case3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        let expected = false;
        assert_eq!(expected, Solution::exist(board, word));
    }
}
