pub struct Solution;

impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 1 {
                    if i > 0 && j > 0 {
                        matrix[i][j] =
                            1 + matrix[i - 1][j].min(matrix[i][j - 1].min(matrix[i - 1][j - 1]));
                    }
                    result += matrix[i][j];
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
        let matrix = [[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(15, Solution::count_squares(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(7, Solution::count_squares(matrix));
    }
}
