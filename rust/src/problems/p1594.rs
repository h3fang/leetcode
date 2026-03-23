pub struct Solution;

const MOD: i64 = 10_0000_0007;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid[0].len();
        let mut f = vec![[0; 2]; n];
        for (i, r) in grid.iter().enumerate() {
            for (j, &x) in r.iter().enumerate() {
                let x = x as i64;
                if i == 0 && j == 0 {
                    f[0] = [x, x];
                } else {
                    let (mut min, mut max) = (i64::MAX, i64::MIN);
                    if i > 0 {
                        min = (f[j][0] * x).min(f[j][1] * x);
                        max = (f[j][0] * x).max(f[j][1] * x);
                    }
                    if j > 0 {
                        min = min.min(f[j - 1][0] * x).min(f[j - 1][1] * x);
                        max = max.max(f[j - 1][0] * x).max(f[j - 1][1] * x);
                    }
                    f[j] = [min, max];
                }
            }
        }
        let f = f[n - 1][1];
        if f < 0 { -1 } else { (f % MOD) as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[-1, -2, -3], [-2, -3, -3], [-3, -3, -2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(-1, Solution::max_product_path(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, -2, 1], [1, -2, 1], [3, -4, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(8, Solution::max_product_path(grid));
    }

    #[test]
    fn case3() {
        let grid = [[1, 3], [0, -4]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(0, Solution::max_product_path(grid));
    }
}
