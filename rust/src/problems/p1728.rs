pub struct Solution;

use std::collections::VecDeque;

const UNKNOWN: u8 = 0;
const CAT_WIN: u8 = 1;
const MOUSE_WIN: u8 = 2;
const CAT: u8 = 0;
const MOUSE: u8 = 1;

struct Grid {
    grid: Vec<char>,
    n: i8,
    size: i8,
    cat: i8,
    mouse: i8,
    food: i8,
    cat_jump: i8,
    mouse_jump: i8,
}

impl Grid {
    fn from_strings(grid: Vec<String>, cat_jump: i8, mouse_jump: i8) -> Self {
        let m = grid.len() as i8;
        let n = grid[0].len() as i8;
        let size = m * n;

        let mut cat = -1;
        let mut mouse = -1;
        let mut food = -1;
        let grid = grid.iter().flat_map(|s| s.chars()).collect::<Vec<_>>();

        for i in 0..m {
            for j in 0..n {
                let idx = i * n + j;
                match grid[idx as usize] {
                    'C' => cat = idx,
                    'M' => mouse = idx,
                    'F' => food = idx,
                    _ => {}
                }
            }
        }

        Self {
            grid,
            n,
            size,
            cat,
            mouse,
            food,
            cat_jump,
            mouse_jump,
        }
    }

    fn prev_moves(&self, c: i8, m: i8, turn: u8) -> Vec<i8> {
        let (jump_length, p0) = if turn == CAT {
            (self.mouse_jump, m)
        } else {
            (self.cat_jump, c)
        };
        let mut result = Vec::with_capacity(jump_length as usize * 4);
        result.push(p0);
        let (r, c) = (p0 / self.n, p0 % self.n);
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            for k in 1..=jump_length {
                let r1 = r + dr * k;
                let c1 = c + dc * k;
                if r1 < 0 || c1 < 0 || r1 >= self.size / self.n || c1 >= self.n {
                    break;
                }
                let p1 = r1 * self.n + c1;
                if self.grid[p1 as usize] == '#' {
                    break;
                }
                result.push(p1);
            }
        }
        result
    }
}

#[derive(Default, Clone, Copy)]
struct Info {
    winning: u8,
    degree: u8,
    steps: u16,
}

impl Solution {
    pub fn can_mouse_win(grid: Vec<String>, cat_jump: i32, mouse_jump: i32) -> bool {
        let g = Grid::from_strings(grid, cat_jump as i8, mouse_jump as i8);

        let mut states = [[[Info::default(); 2]; 64]; 64];
        let mut q = VecDeque::new();

        for c in 0..g.size {
            for m in 0..g.size {
                if g.grid[c as usize] == '#' || g.grid[m as usize] == '#' {
                    continue;
                }
                let winning = if c == m || c == g.food {
                    CAT_WIN
                } else if m == g.food && c != m {
                    MOUSE_WIN
                } else {
                    UNKNOWN
                };
                if winning != UNKNOWN {
                    states[c as usize][m as usize][0].winning = winning;
                    states[c as usize][m as usize][1].winning = winning;
                    q.push_back((c, m, CAT));
                    q.push_back((c, m, MOUSE));
                } else {
                    states[c as usize][m as usize][0].degree =
                        g.prev_moves(c, m, MOUSE).len() as u8;
                    states[c as usize][m as usize][1].degree = g.prev_moves(c, m, CAT).len() as u8;
                }
            }
        }

        while let Some((c, m, turn)) = q.pop_front() {
            let prev_pos = g.prev_moves(c, m, turn);
            let prev_turn = 1 - turn;
            let (winning, steps) = {
                let s = &states[c as usize][m as usize][turn as usize];
                (s.winning, s.steps)
            };

            prev_pos.iter().for_each(|&p| {
                if prev_turn == CAT {
                    let prev_state = &mut states[p as usize][m as usize][CAT as usize];
                    if prev_state.winning != UNKNOWN {
                        return;
                    }
                    if winning == CAT_WIN {
                        prev_state.winning = CAT_WIN;
                        prev_state.steps = steps + 1;
                        q.push_back((p, m, CAT));
                    } else {
                        prev_state.degree -= 1;
                        if prev_state.degree == 0 {
                            prev_state.winning = MOUSE_WIN;
                            prev_state.steps = steps + 1;
                            q.push_back((p, m, CAT));
                        }
                    }
                } else {
                    let prev_state = &mut states[c as usize][p as usize][MOUSE as usize];
                    if prev_state.winning != UNKNOWN {
                        return;
                    }
                    if winning == MOUSE_WIN {
                        prev_state.winning = MOUSE_WIN;
                        prev_state.steps = steps + 1;
                        q.push_back((c, p, MOUSE));
                    } else {
                        prev_state.degree -= 1;
                        if prev_state.degree == 0 {
                            prev_state.winning = CAT_WIN;
                            prev_state.steps = steps + 1;
                            q.push_back((c, p, MOUSE));
                        }
                    }
                }
            });
        }
        let s = &states[g.cat as usize][g.mouse as usize][MOUSE as usize];
        s.winning == MOUSE_WIN && s.steps <= 1000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = ["####F", "#C...", "M...."];
        let grid = grid.iter().map(|r| r.to_string()).collect();
        assert_eq!(true, Solution::can_mouse_win(grid, 1, 2));
    }

    #[test]
    fn case2() {
        let grid = ["M.C...F"];
        let grid = grid.iter().map(|r| r.to_string()).collect();
        assert_eq!(true, Solution::can_mouse_win(grid, 1, 4));
    }

    #[test]
    fn case3() {
        let grid = ["M.C...F"];
        let grid = grid.iter().map(|r| r.to_string()).collect();
        assert_eq!(false, Solution::can_mouse_win(grid, 1, 3));
    }

    #[test]
    fn case4() {
        let grid = ["C...#", "...#F", "....#", "M...."];
        let grid = grid.iter().map(|r| r.to_string()).collect();
        assert_eq!(false, Solution::can_mouse_win(grid, 2, 5));
    }

    #[test]
    fn case5() {
        let grid = [".M...", "..#..", "#..#.", "C#.#.", "...#F"];
        let grid = grid.iter().map(|r| r.to_string()).collect();
        assert_eq!(true, Solution::can_mouse_win(grid, 3, 1));
    }
}
