pub struct NumMatrix {
    cache: Vec<Vec<i32>>,
}

impl NumMatrix {
    pub fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut cache = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                cache[i + 1][j + 1] =
                    cache[i + 1][j] + cache[i][j + 1] - cache[i][j] + matrix[i][j];
            }
        }
        Self { cache }
    }

    pub fn sum_region(&mut self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let row1 = row1 as usize;
        let row2 = row2 as usize;
        let col1 = col1 as usize;
        let col2 = col2 as usize;
        self.cache[row2 + 1][col2 + 1] - self.cache[row1][col2 + 1] - self.cache[row2 + 1][col1]
            + self.cache[row1][col1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ];
        let mut num_matrix = NumMatrix::new(matrix);
        assert_eq!(8, num_matrix.sum_region(2, 1, 4, 3));
        assert_eq!(11, num_matrix.sum_region(1, 1, 2, 2));
        assert_eq!(12, num_matrix.sum_region(1, 2, 2, 4));
    }
}
