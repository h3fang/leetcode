pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while let Some((i, j)) = q.pop_front() {
            result.push(nums[i][j]);
            if j == 0 && i + 1 < nums.len() {
                q.push_back((i + 1, j));
            }
            if j + 1 < nums[i].len() {
                q.push_back((i, j + 1));
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
        let nums = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(
            vec![1, 4, 2, 7, 5, 3, 8, 6, 9],
            Solution::find_diagonal_order(nums)
        );
    }

    #[test]
    fn case2() {
        let nums = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16],
        ];
        assert_eq!(
            vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16],
            Solution::find_diagonal_order(nums)
        );
    }
}
