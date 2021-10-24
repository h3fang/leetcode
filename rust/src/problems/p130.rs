pub struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn dfs(board: &mut Vec<Vec<char>>, x: usize, y: usize) {
            if board[x][y] == 'O' {
                let m = board.len();
                let n = board[0].len();
                board[x][y] = 'A';

                if x > 0 {
                    dfs(board, x - 1, y);
                }
                if x < m - 1 {
                    dfs(board, x + 1, y);
                }
                if y > 0 {
                    dfs(board, x, y - 1);
                }
                if y < n - 1 {
                    dfs(board, x, y + 1);
                }
            }
        }

        let m = board.len();
        let n = board[0].len();
        for x in 0..m {
            dfs(board, x, 0);
            dfs(board, x, n - 1);
        }

        for y in 0..n {
            dfs(board, 0, y);
            dfs(board, m - 1, y);
        }

        board.iter_mut().flatten().for_each(|e| {
            if *e == 'O' {
                *e = 'X';
            } else if *e == 'A' {
                *e = 'O';
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let board = [
            ["X", "X", "X", "X"],
            ["X", "O", "O", "X"],
            ["X", "X", "O", "X"],
            ["X", "O", "X", "X"],
        ];
        let mut board = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let expected = [
            ["X", "X", "X", "X"],
            ["X", "X", "X", "X"],
            ["X", "X", "X", "X"],
            ["X", "O", "X", "X"],
        ];
        let expected = expected
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case2() {
        let board = [["X"]];
        let mut board = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let expected = [["X"]];
        let expected = expected
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case3() {
        let board = [
            ["X", "X", "X", "X", "O", "X"],
            ["O", "X", "X", "O", "O", "X"],
            ["X", "O", "X", "O", "O", "O"],
            ["X", "O", "O", "O", "X", "O"],
            ["O", "O", "X", "X", "O", "X"],
            ["X", "O", "X", "O", "X", "X"],
        ];
        let mut board = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let expected = [
            ["X", "X", "X", "X", "O", "X"],
            ["O", "X", "X", "O", "O", "X"],
            ["X", "O", "X", "O", "O", "O"],
            ["X", "O", "O", "O", "X", "O"],
            ["O", "O", "X", "X", "X", "X"],
            ["X", "O", "X", "O", "X", "X"],
        ];
        let expected = expected
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }

    #[test]
    fn case4() {
        let board = [
            ["X", "X", "X", "X", "O", "O", "X", "X", "O"],
            ["O", "O", "O", "O", "X", "X", "O", "O", "X"],
            ["X", "O", "X", "O", "O", "X", "X", "O", "X"],
            ["O", "O", "X", "X", "X", "O", "O", "O", "O"],
            ["X", "O", "O", "X", "X", "X", "X", "X", "O"],
            ["O", "O", "X", "O", "X", "O", "X", "O", "X"],
            ["O", "O", "O", "X", "X", "O", "X", "O", "X"],
            ["O", "O", "O", "X", "O", "O", "O", "X", "O"],
            ["O", "X", "O", "O", "O", "X", "O", "X", "O"],
        ];
        let mut board = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let expected = [
            ["X", "X", "X", "X", "O", "O", "X", "X", "O"],
            ["O", "O", "O", "O", "X", "X", "O", "O", "X"],
            ["X", "O", "X", "O", "O", "X", "X", "O", "X"],
            ["O", "O", "X", "X", "X", "O", "O", "O", "O"],
            ["X", "O", "O", "X", "X", "X", "X", "X", "O"],
            ["O", "O", "X", "X", "X", "O", "X", "X", "X"],
            ["O", "O", "O", "X", "X", "O", "X", "X", "X"],
            ["O", "O", "O", "X", "O", "O", "O", "X", "O"],
            ["O", "X", "O", "O", "O", "X", "O", "X", "O"],
        ];
        let expected = expected
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| c.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Solution::solve(&mut board);
        assert_eq!(expected, board);
    }
}
