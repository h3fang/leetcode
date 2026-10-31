pub struct Solution;

const MOD: i32 = 10_0000_0007;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();

        let mut f0 = vec![0; n + 1];
        let mut f1 = vec![0; n + 1];

        f0[1] = 1;
        f1[1] = 1;

        for row in grid {
            for (j, x) in row.into_iter().enumerate() {
                if x == 0 {
                    f0[j + 1] = (f0[j] + f1[j + 1]) % MOD;
                    f1[j + 1] = f0[j + 1];
                } else {
                    f0[j + 1] = f1[j + 1];
                    f1[j + 1] = f0[j];
                }
            }
        }

        f0[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1, 0], [0, 0, 1], [1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(5, Solution::unique_paths(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0], [0, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::unique_paths(grid));
    }

    #[test]
    fn case3() {
        let grid = [[0, 1, 1], [1, 1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::unique_paths(grid));
    }
}
