pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = grid[0].len();
        if k > 0 && grid[0][0] >= grid.last().unwrap()[n - 1] {
            return 0;
        }

        let max = *grid.iter().flatten().max().unwrap() as usize;
        let mut suf = vec![i32::MAX; max + 2];
        let mut f = vec![i32::MAX; n + 1];
        for _ in 0..k + 1 {
            let mut min = vec![i32::MAX; max + 1];
            f.iter_mut().for_each(|x| *x = i32::MAX);
            f[1] = -grid[0][0];
            for r in &grid {
                for (j, &x) in r.iter().enumerate() {
                    f[j + 1] = (f[j].min(f[j + 1]) + x).min(suf[x as usize]);
                    min[x as usize] = min[x as usize].min(f[j + 1]);
                }
            }

            let old = suf.clone();
            for (i, &x) in min.iter().enumerate().rev() {
                suf[i] = suf[i + 1].min(x);
            }
            if suf == old {
                break;
            }
        }
        f[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 3, 3], [2, 5, 4], [4, 3, 5]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(7, Solution::min_cost(grid, 2));
    }

    #[test]
    fn case2() {
        let grid = [[1, 2], [2, 3], [3, 4]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(9, Solution::min_cost(grid, 1));
    }
}
