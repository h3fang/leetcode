pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        fn dfs(m: i32, n: i32, f: &mut [Vec<Vec<i32>>], i: i32, j: i32, max_move: i32) -> i32 {
            if i < 0 || j < 0 || i == m || j == n {
                return 1;
            }
            if max_move == 0 {
                return 0;
            }
            if f[i as usize][j as usize][max_move as usize] >= 0 {
                return f[i as usize][j as usize][max_move as usize];
            }

            let mut c = 0;
            for &(i1, j1) in &[(i, j + 1), (i, j - 1), (i + 1, j), (i - 1, j)] {
                c = (c + dfs(m, n, f, i1, j1, max_move - 1)) % MOD;
            }
            f[i as usize][j as usize][max_move as usize] = c;
            c
        }

        let mut f = vec![vec![vec![-1; max_move as usize + 1]; n as usize]; m as usize];
        dfs(m, n, &mut f, start_row, start_column, max_move)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::find_paths(2, 2, 2, 0, 0));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::find_paths(1, 3, 3, 0, 1));
    }
}
