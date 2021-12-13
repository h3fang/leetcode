pub struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut mr = vec![0; m];
        for i in 0..m {
            mr[i] = *grid[i].iter().max().unwrap();
        }

        let mut mc = vec![0; n];
        for j in 0..n {
            mc[j] = grid.iter().map(|r| r[j]).max().unwrap();
        }

        let mut result = 0;
        for (i, row) in grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                result += mr[i].min(mc[j]) - cell;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[3, 0, 8, 4], [2, 4, 5, 7], [9, 2, 6, 3], [0, 3, 1, 0]];
        let grid = grid.iter().map(|row| row.to_vec()).collect();
        assert_eq!(35, Solution::max_increase_keeping_skyline(grid));
    }
}
