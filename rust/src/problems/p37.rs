pub struct Solution;

struct State<'a> {
    board: &'a mut [Vec<char>],
    row: [u16; 9],
    column: [u16; 9],
    block: [u16; 9],
}

impl<'a> State<'a> {
    fn new(board: &'a mut Vec<Vec<char>>) -> Self {
        let mut s = State {
            board,
            row: Default::default(),
            column: Default::default(),
            block: Default::default(),
        };
        for (i, r) in s.board.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == '.' {
                    continue;
                }
                let d = c as u8 - b'0';
                s.row[i] |= 1 << d;
                s.column[j] |= 1 << d;
                s.block[(i / 3) * 3 + j / 3] |= 1 << d;
            }
        }
        s
    }

    fn find_cell_with_least_candidates(&self) -> Option<(usize, usize)> {
        let mut min = (9, 9, 9);
        for (i, r) in self.board.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c != '.' {
                    continue;
                }
                let filled = self.row[i] | self.column[j] | self.block[(i / 3) * 3 + j / 3];
                let candidates = 9 - filled.count_ones();
                if candidates < min.0 {
                    min = (candidates, i, j);
                }
                if min.0 == 1 {
                    return Some((min.1, min.2));
                }
            }
        }
        if min.1 != 9 {
            Some((min.1, min.2))
        } else {
            None
        }
    }

    fn solve(&mut self) -> bool {
        let Some((i, j)) = self.find_cell_with_least_candidates() else {
            return true;
        };
        let filled = self.row[i] | self.column[j] | self.block[(i / 3) * 3 + j / 3];
        for d in 1..=9 {
            if (1 << d) & filled > 0 {
                continue;
            }
            self.board[i][j] = (b'0' + d) as char;
            self.row[i] |= 1 << d;
            self.column[j] |= 1 << d;
            self.block[(i / 3) * 3 + j / 3] |= 1 << d;
            if self.solve() {
                return true;
            }
            let mask = !(1 << d);
            self.row[i] &= mask;
            self.column[j] &= mask;
            self.block[(i / 3) * 3 + j / 3] &= mask;
        }
        self.board[i][j] = '.';
        false
    }
}

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut s = State::new(board);
        s.solve();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut board = [
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
        .map(|r| r.iter().map(|s| s.as_bytes()[0] as char).collect())
        .collect();
        let expected = [
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ]
        .iter()
        .map(|r| {
            r.iter()
                .map(|s| s.as_bytes()[0] as char)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
        Solution::solve_sudoku(&mut board);
        assert_eq!(expected, board);
    }
}
