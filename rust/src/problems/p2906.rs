pub struct Solution;

const MOD: i64 = 12345;

impl Solution {
    pub fn construct_product_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut suf = vec![1; m * n + 1];
        for (i, r) in grid.iter().enumerate().rev() {
            for (j, &c) in r.iter().enumerate().rev() {
                suf[i * n + j] = (suf[i * n + j + 1] * c as i64) % MOD;
            }
        }

        let mut pre = 1;
        for (i, r) in grid.iter_mut().enumerate() {
            for (j, c) in r.iter_mut().enumerate() {
                let v = *c as i64;
                *c = ((suf[i * n + j + 1] * pre) % MOD) as i32;
                pre = (pre * v) % MOD;
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 2], [3, 4]].iter().map(|r| r.to_vec()).collect();
        let expected = [[24, 12], [8, 6]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::construct_product_matrix(grid));
    }

    #[test]
    fn case2() {
        let grid = [[12345], [2], [1]].iter().map(|r| r.to_vec()).collect();
        let expected = [[2], [0], [0]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::construct_product_matrix(grid));
    }
}
