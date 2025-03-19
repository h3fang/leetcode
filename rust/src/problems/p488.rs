use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    fn remove(board: &[u8]) -> Vec<u8> {
        let n = board.len();
        let mut s = Vec::with_capacity(n);
        let mut count = 0;
        for b in board {
            if !s.is_empty() && b == s.last().unwrap() {
                count += 1;
            } else {
                if count >= 3 {
                    for _ in 0..count {
                        s.pop();
                    }
                }
                if !s.is_empty() && s.last().unwrap() == b {
                    let n = s.len();
                    if n > 1 && s[n - 2] == *b {
                        count = 3;
                    } else {
                        count = 2;
                    }
                } else {
                    count = 1;
                }
            }
            s.push(*b);
        }
        if count >= 3 {
            for _ in 0..count {
                s.pop();
            }
        }
        s
    }

    pub fn find_min_step(board: String, hand: String) -> i32 {
        let mut board = board.into_bytes();
        let mut hand = hand.into_bytes();
        hand.sort_unstable();

        fn dfs(board: &mut Vec<u8>, hand: &mut [u8], curr: i32, result: &mut i32) {
            if board.is_empty() {
                *result = (*result).min(curr);
            } else if !hand.is_empty() && curr + 1 < *result {
                for i in 0..=board.len() {
                    for j in 0..hand.len() {
                        if j > 0 && hand[j] == hand[j - 1] {
                            continue;
                        }
                        if i > 0 && hand[j] == board[i - 1] {
                            continue;
                        }

                        if i < board.len() && board[i] == hand[j] {
                            board.insert(i, hand[j]);
                            hand.swap(0, j);
                            let mut new_board = if i + 1 < board.len() && board[i] == board[i + 1] {
                                Solution::remove(board)
                            } else {
                                board.clone()
                            };
                            dfs(&mut new_board, &mut hand[1..], curr + 1, result);
                            hand.swap(0, j);
                            board.remove(i);
                        }

                        if i > 0
                            && i < board.len()
                            && board[i] == board[i - 1]
                            && board[i] != hand[j]
                        {
                            let mut new_board = board.clone();
                            new_board.insert(i, hand[j]);
                            hand.swap(0, j);
                            dfs(&mut new_board, &mut hand[1..], curr + 1, result);
                            hand.swap(0, j);
                        }
                    }
                }
            }
        }
        let mut result = i32::MAX;
        dfs(&mut board, &mut hand, 0, &mut result);
        if result == i32::MAX { -1 } else { result }
    }

    pub fn find_min_step_bfs(board: String, hand: String) -> i32 {
        let board = board.into_bytes();
        let mut hand = hand.into_bytes();
        hand.sort_unstable();

        let mut q = VecDeque::with_capacity(board.len() * hand.len());
        q.push_back((board, hand, 0));
        while let Some((mut board, hand, count)) = q.pop_front() {
            if board.is_empty() {
                return count;
            } else if hand.is_empty() {
                continue;
            } else {
                for i in 0..=board.len() {
                    for j in 0..hand.len() {
                        if j > 0 && hand[j] == hand[j - 1] {
                            continue;
                        }
                        if i > 0 && hand[j] == board[i - 1] {
                            continue;
                        }

                        if i < board.len() && board[i] == hand[j] {
                            board.insert(i, hand[j]);
                            let new_board = if i + 1 < board.len() && board[i] == board[i + 1] {
                                Solution::remove(&board)
                            } else {
                                board.clone()
                            };
                            let mut new_hand = hand.clone();
                            new_hand.remove(j);
                            q.push_back((new_board, new_hand, count + 1));
                            board.remove(i);
                        }

                        if i > 0
                            && i < board.len()
                            && board[i] == board[i - 1]
                            && board[i] != hand[j]
                        {
                            let mut new_board = board.clone();
                            new_board.insert(i, hand[j]);
                            let mut new_hand = hand.clone();
                            new_hand.remove(j);
                            q.push_back((new_board, new_hand, count + 1));
                        }
                    }
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
        let board = "WRRBBW".to_string();
        let hand = "RB".to_string();
        assert_eq!(-1, Solution::find_min_step(board, hand));
    }

    #[test]
    fn case2() {
        let board = "WWRRBBWW".to_string();
        let hand = "WRBRW".to_string();
        assert_eq!(2, Solution::find_min_step(board, hand));
    }

    #[test]
    fn case3() {
        let board = "G".to_string();
        let hand = "GGGGG".to_string();
        assert_eq!(2, Solution::find_min_step(board, hand));
    }

    #[test]
    fn case4() {
        let board = "RBYYBBRRB".to_string();
        let hand = "YRBGB".to_string();
        assert_eq!(3, Solution::find_min_step(board, hand));
    }

    #[test]
    fn case5() {
        let board = "RRWWRRBBRR".to_string();
        let hand = "WB".to_string();
        assert_eq!(2, Solution::find_min_step(board, hand));
    }

    #[test]
    fn case6() {
        let board = "RRYGGYYRRYGGYYRR".to_string();
        let hand = "GGBBB".to_string();
        assert_eq!(5, Solution::find_min_step(board, hand));
    }
}
