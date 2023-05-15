pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut m = HashMap::new();
        for mut r in matrix {
            if r[0] == 1 {
                r.iter_mut().for_each(|c| *c = 1 - *c);
            }
            *m.entry(r).or_insert(0) += 1;
        }
        *m.values().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[0, 1], [1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(1, Solution::max_equal_rows_after_flips(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[0, 1], [1, 0]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::max_equal_rows_after_flips(matrix));
    }

    #[test]
    fn case3() {
        let matrix = [[0, 0, 0], [0, 0, 1], [1, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::max_equal_rows_after_flips(matrix));
    }
}
