pub struct Solution;

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for (i, row) in grid.iter().enumerate() {
            if row[ans] == 1 {
                ans = i;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[0, 1], [0, 0]].iter().map(|g| g.to_vec()).collect();
        assert_eq!(0, Solution::find_champion(grid));
    }

    #[test]
    fn case2() {
        let grid = [[0, 0, 1], [1, 0, 1], [0, 0, 0]]
            .iter()
            .map(|g| g.to_vec())
            .collect();
        assert_eq!(1, Solution::find_champion(grid));
    }
}
