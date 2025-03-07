pub struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut m = vec![false; grid.len() * grid.len() + 1];
        let mut a = 0;
        for x in grid.into_iter().flatten() {
            if m[x as usize] {
                a = x;
            } else {
                m[x as usize] = true;
            }
        }
        let b = m
            .iter()
            .enumerate()
            .skip(1)
            .find(|e| !e.1)
            .map(|e| e.0 as i32)
            .unwrap();
        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let grid = [[1, 3], [2, 2]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(vec![2, 4], Solution::find_missing_and_repeated_values(grid));
    }

    #[test]
    fn case2() {
        let grid = [[9, 1, 7], [8, 9, 2], [3, 4, 6]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(vec![9, 5], Solution::find_missing_and_repeated_values(grid));
    }
}
