pub struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let (mut i, mut j) = (0, 0);
        for (i1, r) in board.iter().enumerate() {
            for (j1, &c) in r.iter().enumerate() {
                if c == 'R' {
                    (i, j) = (i1 as i32, j1 as i32);
                }
            }
        }
        for (di, dj) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
            let (mut i1, mut j1) = (i, j);
            for _ in 1.. {
                i1 += di;
                j1 += dj;
                if i1 < 0 || j1 < 0 || i1 == 8 || j1 == 8 {
                    break;
                }
                let c = board[i1 as usize][j1 as usize];
                if c == 'p' {
                    result += 1;
                }
                if c != '.' {
                    break;
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
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", "p", ".", ".", ".", "."],
            [".", ".", ".", "R", ".", ".", ".", "p"],
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", "p", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(3, Solution::num_rook_captures(board));
    }

    #[test]
    fn case2() {
        let board = [
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", "p", "p", "p", "p", "p", ".", "."],
            [".", "p", "p", "B", "p", "p", ".", "."],
            [".", "p", "B", "R", "B", "p", ".", "."],
            [".", "p", "p", "B", "p", "p", ".", "."],
            [".", "p", "p", "p", "p", "p", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(0, Solution::num_rook_captures(board));
    }

    #[test]
    fn case3() {
        let board = [
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", "p", ".", ".", ".", "."],
            [".", ".", ".", "p", ".", ".", ".", "."],
            ["p", "p", ".", "R", ".", "p", "B", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
            [".", ".", ".", "B", ".", ".", ".", "."],
            [".", ".", ".", "p", ".", ".", ".", "."],
            [".", ".", ".", ".", ".", ".", ".", "."],
        ]
        .iter()
        .map(|r| r.iter().map(|c| c.chars().next().unwrap()).collect())
        .collect();
        assert_eq!(3, Solution::num_rook_captures(board));
    }
}
