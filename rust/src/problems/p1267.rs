pub struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![0; grid.len()];
        let mut col = vec![0; grid[0].len()];
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }
        let mut result = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, &c) in r.iter().enumerate() {
                if c == 1 && (row[i] > 1 || col[j] > 1) {
                    result += 1;
                }
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
        let grid = [[1, 0], [1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::count_servers(grid));
    }

    #[test]
    fn case2() {
        let grid = [[1, 1, 0, 0], [0, 0, 1, 0], [0, 0, 1, 0], [0, 0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::count_servers(grid));
    }
}
