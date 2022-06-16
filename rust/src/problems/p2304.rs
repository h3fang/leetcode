pub struct Solution;

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = grid[m - 1].clone();
        for row in grid.iter().rev().skip(1) {
            let mut next = vec![0; n];
            for (i, &cell) in row.iter().enumerate() {
                let mut min = i32::MAX;
                for (j, d) in dp.iter().enumerate() {
                    let cost = d + move_cost[cell as usize][j];
                    min = min.min(cost);
                }
                next[i] = cell + min;
            }
            dp = next;
        }
        *dp.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_vec(arr: &[[i32; 2]]) -> Vec<Vec<i32>> {
        arr.iter().map(|a| a.to_vec()).collect()
    }

    #[test]
    fn case1() {
        let grid = parse_vec(&[[5, 3], [4, 0], [2, 1]]);
        let move_cost = parse_vec(&[[9, 8], [1, 5], [10, 12], [18, 6], [2, 4], [14, 3]]);
        assert_eq!(17, Solution::min_path_cost(grid, move_cost));
    }
}
