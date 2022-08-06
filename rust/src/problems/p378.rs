pub struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        fn is_more_than_k(matrix: &[Vec<i32>], mid: i32, k: i32) -> bool {
            let n = matrix.len() as i32;
            let mut count = 0;
            let mut i = n - 1;
            let mut j = 0;
            while i >= 0 && j < n {
                if matrix[i as usize][j as usize] <= mid {
                    count += i + 1;
                    j += 1;
                } else {
                    i -= 1;
                }
            }
            count >= k
        }
        let n = matrix.len();
        let mut left = matrix[0][0];
        let mut right = matrix[n - 1][n - 1];
        while left < right {
            let mid = left + (right - left) / 2;
            if is_more_than_k(&matrix, mid, k) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, 5, 9], [10, 11, 13], [12, 13, 15]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let k = 8;
        assert_eq!(13, Solution::kth_smallest(matrix, k));
    }

    #[test]
    fn case2() {
        let matrix = [[-5, -4], [-5, -4]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let k = 2;
        assert_eq!(-5, Solution::kth_smallest(matrix, k));
    }
}
