use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let get = |idx: usize| matrix[idx / n][idx % n];

        let mut left = 0i32;
        let mut right = (m * n - 1) as i32;
        while left >= 0 && right < (m * n) as i32 && left <= right {
            let c = left + (right - left) / 2;
            match get(c as usize).cmp(&target) {
                Ordering::Greater => right = c - 1,
                Ordering::Less => left = c + 1,
                Ordering::Equal => return true,
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(true, Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn case2() {
        let matrix = vec![vec![1]];
        assert_eq!(false, Solution::search_matrix(matrix, 0));
    }
}
