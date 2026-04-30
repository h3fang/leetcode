pub struct Solution;

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, k) = (grid[0].len(), k as usize);

        let mut f = vec![vec![i32::MIN / 2; k + 2]; n + 1];
        f[1].iter_mut().skip(1).for_each(|f| *f = 0);

        for r in grid {
            for (j, c) in r.into_iter().enumerate() {
                for q in (1..k + 2).rev() {
                    let q1 = if c > 0 { q - 1 } else { q };
                    f[j + 1][q] = f[j + 1][q1].max(f[j][q1]) + c;
                }
            }
        }

        let f = f[n][k + 1];
        if f < 0 { -1 } else { f }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1], [2, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::max_path_score(grid, 1));
    }

    #[test]
    fn case2() {
        let grid = [[0, 1], [1, 2]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(-1, Solution::max_path_score(grid, 1));
    }
}
