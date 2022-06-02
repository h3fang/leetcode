pub struct Solution;

impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = vec![vec![0; m]; n];
        for (i, row) in matrix.iter().enumerate() {
            for (j, &e) in row.iter().enumerate() {
                result[j][i] = e;
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
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let result = [[1, 4, 7], [2, 5, 8], [3, 6, 9]];
        let result = result.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        assert_eq!(result, Solution::transpose(matrix));
    }
}
