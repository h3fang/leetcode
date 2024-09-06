pub struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        (0..m).all(|i| {
            let num = matrix[i][0];
            (1..(m - i).min(n)).all(|d| matrix[i + d][d] == num)
        }) && (1..n).all(|j| {
            let num = matrix[0][j];
            (1..(n - j).min(m)).all(|d| matrix[d][j + d] == num)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert!(Solution::is_toeplitz_matrix(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[1, 2], [2, 2]].iter().map(|r| r.to_vec()).collect();
        assert!(!Solution::is_toeplitz_matrix(matrix));
    }
}
