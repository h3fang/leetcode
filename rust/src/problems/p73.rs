pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut first_col = false;
        for i in 0..m {
            if matrix[i][0] == 0 {
                first_col = true;
            }
            for j in 1..n {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..m {
            for j in 1..n {
                if matrix[0][j] == 0 || matrix[i][0] == 0 {
                    matrix[i][j] = 0;
                }
            }
        }

        if matrix[0][0] == 0 {
            matrix[0].iter_mut().skip(1).for_each(|e| *e = 0);
        }

        if first_col {
            matrix.iter_mut().for_each(|r| r[0] = 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, 1, 1], [1, 0, 1], [1, 1, 1]];
        let mut matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let expected = [[1, 0, 1], [0, 0, 0], [1, 0, 1]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        Solution::set_zeroes(&mut matrix);

        assert_eq!(expected, matrix);
    }

    #[test]
    fn case2() {
        let matrix = [[0, 1, 2, 0], [3, 4, 5, 2], [1, 3, 1, 5]];
        let mut matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let expected = [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        Solution::set_zeroes(&mut matrix);

        assert_eq!(expected, matrix);
    }
}
