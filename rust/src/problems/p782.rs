pub struct Solution;

impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let row0 = board[0].iter().fold(0, |acc, &e| (acc << 1) | e as u32);
        let col0 = (0..n).fold(0, |acc, i| (acc << 1) | board[i][0] as u32);

        let rev_row0 = ((1 << n) - 1) & !row0;
        let rev_col0 = ((1 << n) - 1) & !col0;

        let mut row_cnt = 1;
        let mut col_cnt = 1;

        for i in 1..n {
            let row = board[i].iter().fold(0, |acc, &e| (acc << 1) | e as u32);
            if row != row0 && row != rev_row0 {
                return -1;
            }
            if row == row0 {
                row_cnt += 1;
            }

            let col = (0..n).fold(0, |acc, j| (acc << 1) | board[j][i] as u32);
            if col != col0 && col != rev_col0 {
                return -1;
            }
            if col == col0 {
                col_cnt += 1;
            }
        }

        fn get_moves(mask: u32, count: u32, n: u32) -> i32 {
            let ones = mask.count_ones();
            if n % 2 == 0 {
                if 2 * count != n || 2 * ones != n {
                    return -1;
                }
                let r0 = n / 2 - (mask & 0xAAAAAAAA).count_ones();
                let r1 = n / 2 - (mask & 0x55555555).count_ones();
                r0.min(r1) as i32
            } else {
                if (n as i32 - 2 * ones as i32).abs() != 1
                    || (n as i32 - 2 * count as i32).abs() != 1
                {
                    return -1;
                }
                if n / 2 == ones {
                    (n / 2 - (mask & 0xAAAAAAAA).count_ones()) as i32
                } else {
                    ((n + 1) / 2 - (mask & 0x55555555).count_ones()) as i32
                }
            }
        }

        let m_row = get_moves(row0, row_cnt, n as u32);
        let m_col = get_moves(col0, col_cnt, n as u32);

        if m_row == -1 || m_col == -1 {
            -1
        } else {
            m_row + m_col
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let board = [[0, 1, 1, 0], [0, 1, 1, 0], [1, 0, 0, 1], [1, 0, 0, 1]];
        let board = board.iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::moves_to_chessboard(board));
    }

    #[test]
    fn case2() {
        let board = [[0, 1], [1, 0]];
        let board = board.iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::moves_to_chessboard(board));
    }

    #[test]
    fn case3() {
        let board = [[1, 0], [1, 0]];
        let board = board.iter().map(|r| r.to_vec()).collect();
        assert_eq!(-1, Solution::moves_to_chessboard(board));
    }

    #[test]
    fn case4() {
        let board = [[1, 1, 0], [0, 0, 1], [0, 0, 1]];
        let board = board.iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::moves_to_chessboard(board));
    }
}
