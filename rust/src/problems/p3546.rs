pub struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let mut presum = Vec::with_capacity(grid.len());

        for r in &grid {
            let sum: i64 = r.iter().map(|&e| e as i64).sum();
            presum.push(*presum.last().unwrap_or(&0) + sum);
        }
        let total = *presum.last().unwrap();
        for &s in &presum {
            if 2 * s == total {
                return true;
            }
        }

        presum.clear();
        presum.reserve(grid[0].len());

        for i in 0..grid[0].len() {
            let sum: i64 = grid.iter().map(|r| r[i] as i64).sum();
            presum.push(*presum.last().unwrap_or(&0) + sum);
        }
        for &s in &presum {
            if 2 * s == total {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 4], [2, 3]].iter().map(|r| r.to_vec()).collect();
        assert!(Solution::can_partition_grid(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 3], [2, 4]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::can_partition_grid(grid));
    }
}
