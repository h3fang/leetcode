pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        fn is_valid(queens: &[usize], col: usize) -> bool {
            queens.iter().all(|&c| c != col)
                && queens.iter().rev().enumerate().all(|(i, &c)| {
                    let c = c as i32;
                    let col = col as i32;
                    let i = i as i32;
                    c != col - i - 1 && c != col + i + 1
                })
        }

        fn dfs(queens: &mut Vec<usize>, row: usize, n: usize, result: &mut Vec<Vec<String>>) {
            if row == n {
                let mut board = Vec::with_capacity(n);
                for i in queens {
                    let mut row = vec!['.'; n];
                    row[*i] = 'Q';
                    board.push(row.iter().collect::<String>());
                }
                result.push(board);
                return;
            }
            for col in 0..n {
                if is_valid(queens, col) {
                    queens.push(col);
                    dfs(queens, row + 1, n, result);
                    queens.pop();
                }
            }
        }

        let n = n as usize;
        let mut result = vec![];
        dfs(&mut Vec::with_capacity(n), 0, n, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let expected = [
            [".Q..", "...Q", "Q...", "..Q."],
            ["..Q.", "Q...", "...Q", ".Q.."],
        ];
        let mut expected = expected
            .iter()
            .map(|b| b.iter().map(|r| r.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        expected.sort_unstable();
        let mut result = Solution::solve_n_queens(4);
        result.sort_unstable();
        assert_eq!(expected, result);
    }
}
