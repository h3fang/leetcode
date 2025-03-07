pub struct Solution;

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        (0..grid[0].len())
            .map(|j| grid.iter().map(|r| r[j].to_string().len()).max().unwrap() as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1], [22], [333]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![3], Solution::find_column_width(grid));
    }

    #[test]
    fn case2() {
        let grid = [[-15, 1, 3], [15, 7, 12], [5, 6, -2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(vec![3, 1, 2], Solution::find_column_width(grid));
    }
}
