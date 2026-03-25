pub struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let total: i64 = grid.iter().flatten().map(|&v| v as i64).sum();
        let mut sum = 0;

        for r in &grid {
            sum += r.iter().map(|&e| e as i64).sum::<i64>();
            if sum * 2 == total {
                return true;
            }
            if sum * 2 > total {
                break;
            }
        }

        sum = 0;

        for i in 0..grid[0].len() {
            sum += grid.iter().map(|r| r[i] as i64).sum::<i64>();
            if sum * 2 == total {
                return true;
            }
            if sum * 2 > total {
                break;
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
