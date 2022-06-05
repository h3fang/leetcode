pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        fn is_valid(queens: &[usize], col: usize) -> bool {
            queens.iter().all(|&c| c != col)
                && queens.iter().rev().enumerate().all(|(i, &c)| {
                    let c = c as i32;
                    let col = col as i32;
                    let i = i as i32;
                    c != col - i - 1 && c != col + i + 1
                })
        }

        fn dfs(queens: &mut Vec<usize>, row: usize, n: usize, result: &mut i32) {
            if row == n {
                *result += 1;
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
        let mut result = 0;
        dfs(&mut Vec::with_capacity(n), 0, n, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(352, Solution::total_n_queens(9));
    }
}
