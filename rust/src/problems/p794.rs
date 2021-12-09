pub struct Solution;

impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let board = board
            .into_iter()
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();

        fn is_player_winning(p: u8, b: &[Vec<u8>]) -> bool {
            let mut col = false;
            for i in 0..3 {
                if b.iter().map(|row| row[i]).all(|c| c == p) {
                    col = true;
                    break;
                }
            }
            col || b.iter().any(|row| row.iter().all(|c| *c == p))
                || (b[0][0] == p && b[1][1] == p && b[2][2] == p)
                || (b[0][2] == p && b[1][1] == p && b[2][0] == p)
        }
        let p1 = board.iter().flatten().filter(|c| **c == b'X').count();
        let p2 = board.iter().flatten().filter(|c| **c == b'O').count();
        ((p1 == p2 && !is_player_winning(b'X', &board))
            || (p1 == p2 + 1 && !is_player_winning(b'O', &board)))
            && !(is_player_winning(b'X', &board) && is_player_winning(b'O', &board))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let b = ["XOX", " X ", "   "];
        let board = b.iter().map(|r| r.to_string()).collect();
        assert_eq!(false, Solution::valid_tic_tac_toe(board));
    }

    #[test]
    fn case2() {
        let b = ["XXX", "XOO", "OO "];
        let board = b.iter().map(|r| r.to_string()).collect();
        assert_eq!(false, Solution::valid_tic_tac_toe(board));
    }

    #[test]
    fn case3() {
        let b = ["XOX", "O O", "XOX"];
        let board = b.iter().map(|r| r.to_string()).collect();
        assert_eq!(true, Solution::valid_tic_tac_toe(board));
    }
}
