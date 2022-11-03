pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let m = board.len();
        let n = board[0].len();
        for i in 0..m {
            for j in 0..n {
                let neighbors = (i.saturating_sub(1)..=(i + 1).min(m - 1))
                    .map(|i1| {
                        (j.saturating_sub(1)..=(j + 1).min(n - 1))
                            .map(|j1| i32::from(board[i1][j1].abs() == 1))
                            .sum::<i32>()
                    })
                    .sum::<i32>()
                    - board[i][j];
                if board[i][j] == 1 && neighbors != 2 && neighbors != 3 {
                    board[i][j] = -1;
                } else if board[i][j] == 0 && neighbors == 3 {
                    board[i][j] = 2;
                }
            }
        }

        board.iter_mut().flatten().for_each(|e| {
            if *e == -1 {
                *e = 0;
            } else if *e == 2 {
                *e = 1;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let board = [[0, 1, 0], [0, 0, 1], [1, 1, 1], [0, 0, 0]];
        let mut board = board.iter().map(|row| row.to_vec()).collect();
        let expected = [[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]];
        let expected = expected.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        Solution::game_of_life(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case2() {
        let board = [[1, 1], [1, 0]];
        let mut board = board.iter().map(|row| row.to_vec()).collect();
        let expected = [[1, 1], [1, 1]];
        let expected = expected.iter().map(|row| row.to_vec()).collect::<Vec<_>>();
        Solution::game_of_life(&mut board);
        assert_eq!(expected, board);
    }
}
