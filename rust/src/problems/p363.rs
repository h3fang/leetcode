pub struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut result = i32::MIN;
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 0..m {
            let mut sum = vec![0; n];
            for row in &matrix[i..m] {
                for (k, &c) in row.iter().enumerate() {
                    sum[k] += c;
                }
                let mut set = BTreeSet::new();
                set.insert(0);
                let mut s = 0;
                for &v in &sum {
                    s += v;
                    if let Some(lb) = set.range(s - k..).next() {
                        result = result.max(s - lb);
                    }
                    set.insert(s);
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
        let matrix = [[1, 0, 1], [0, -2, 3]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let k = 2;
        assert_eq!(2, Solution::max_sum_submatrix(matrix, k));
    }

    #[test]
    fn case2() {
        let matrix = [[2, 2, -1]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let k = 3;
        assert_eq!(3, Solution::max_sum_submatrix(matrix, k));
    }
}
