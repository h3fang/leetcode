pub struct Solution;

use std::collections::VecDeque;

const DRAW: u8 = 0;
const MOUSE_WIN: u8 = 1;
const CAT_WIN: u8 = 2;

const MOUSE: u8 = 0;
const CAT: u8 = 1;

impl Solution {
    pub fn cat_mouse_game(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        let mut states = [[[(DRAW, 0i8); 2]; 50]; 50];
        let mut q = VecDeque::new();
        for c in 1..n as u8 {
            for m in 0..n as u8 {
                let winning = if m == 0 {
                    MOUSE_WIN
                } else if c == m {
                    CAT_WIN
                } else {
                    DRAW
                };
                if winning == DRAW {
                    states[m as usize][c as usize][0].1 = graph[m as usize].len() as i8;
                    states[m as usize][c as usize][1].1 =
                        graph[c as usize].iter().filter(|e| **e != 0).count() as i8;
                } else {
                    states[m as usize][c as usize][0].0 = winning;
                    states[m as usize][c as usize][1].0 = winning;
                    q.push_back((m, c, MOUSE));
                    q.push_back((m, c, CAT));
                }
            }
        }
        while let Some((m, c, turn)) = q.pop_front() {
            let winning = states[m as usize][c as usize][turn as usize].0;
            let prev_turn = 1 - turn;
            let node = if prev_turn == MOUSE { m } else { c };
            for &prev in &graph[node as usize] {
                if prev == 0 && prev_turn == CAT {
                    continue;
                }
                let prev_state = if prev_turn == MOUSE {
                    &mut states[prev as usize][c as usize][prev_turn as usize]
                } else {
                    &mut states[m as usize][prev as usize][prev_turn as usize]
                };
                if prev_state.0 > 0 {
                    continue;
                }
                if winning - 1 == prev_turn {
                    prev_state.0 = winning;
                    q.push_back(if prev_turn == MOUSE {
                        (prev as u8, c, prev_turn)
                    } else {
                        (m, prev as u8, prev_turn)
                    });
                } else {
                    prev_state.1 -= 1;
                    if prev_state.1 == 0 {
                        prev_state.0 = if prev_turn == MOUSE {
                            CAT_WIN
                        } else {
                            MOUSE_WIN
                        };
                        q.push_back(if prev_turn == MOUSE {
                            (prev as u8, c, prev_turn)
                        } else {
                            (m, prev as u8, prev_turn)
                        });
                    }
                }
            }
        }
        states[1][2][0].0 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let graph = vec![
            vec![2, 5],
            vec![3],
            vec![0, 4, 5],
            vec![1, 4, 5],
            vec![2, 3],
            vec![0, 2, 3],
        ];
        assert_eq!(0, Solution::cat_mouse_game(graph));
    }

    #[test]
    fn case2() {
        let graph = vec![vec![1, 3], vec![0], vec![3], vec![0, 2]];
        assert_eq!(1, Solution::cat_mouse_game(graph));
    }

    #[test]
    fn case3() {
        let graph = vec![
            vec![5, 6],
            vec![3, 4],
            vec![6],
            vec![1, 4, 5],
            vec![1, 3, 5],
            vec![0, 3, 4, 6],
            vec![0, 2, 5],
        ];
        assert_eq!(2, Solution::cat_mouse_game(graph));
    }
}
