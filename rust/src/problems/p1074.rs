pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        fn sum_to_target(nums: &[i32], target: i32) -> i32 {
            let mut m = HashMap::new();
            m.insert(0, 1);
            let mut pre = 0;
            let mut result = 0;
            for &n in nums {
                pre += n;
                if let Some(c) = m.get(&(pre - target)) {
                    result += c;
                }
                *m.entry(pre).or_insert(0) += 1;
            }
            result
        }

        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = 0;
        for i1 in 0..m {
            let mut sum = vec![0; n];
            for row in &matrix[i1..] {
                for (j, c) in row.iter().enumerate() {
                    sum[j] += c;
                }
                result += sum_to_target(&sum, target);
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
        let matrix = [[0, 1, 0], [1, 1, 1], [0, 1, 0]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let target = 0;
        assert_eq!(4, Solution::num_submatrix_sum_target(matrix, target));
    }

    #[test]
    fn case2() {
        let matrix = [[1, -1], [-1, 1]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let target = 0;
        assert_eq!(5, Solution::num_submatrix_sum_target(matrix, target));
    }
}
