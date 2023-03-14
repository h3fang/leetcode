pub struct Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let (m, n) = (row_sum.len(), col_sum.len());
        let mut result = vec![vec![0; n]; m];
        let (mut i, mut j) = (0, 0);
        while i < m && j < n {
            if row_sum[i] < col_sum[j] {
                result[i][j] = row_sum[i];
                col_sum[j] -= row_sum[i];
                i += 1;
            } else {
                result[i][j] = col_sum[j];
                row_sum[i] -= col_sum[j];
                j += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(matrix: &[Vec<i32>], row_sum: &[i32], col_sum: &[i32]) -> bool {
        matrix
            .iter()
            .zip(row_sum)
            .all(|(row, &sum)| row.iter().sum::<i32>() == sum)
            && col_sum
                .iter()
                .enumerate()
                .all(|(i, &sum)| matrix.iter().map(|r| r[i]).sum::<i32>() == sum)
    }

    #[test]
    fn case1() {
        let row_sum = vec![3, 8];
        let col_sum = vec![4, 7];
        let matrix = Solution::restore_matrix(row_sum.to_vec(), col_sum.to_vec());
        assert!(check(&matrix, &row_sum, &col_sum));
    }
}
