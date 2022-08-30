pub struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        fn transpose(matrix: &mut Vec<Vec<i32>>) {
            let n = matrix.len();
            for i in 0..n {
                for j in i + 1..n {
                    let v = matrix[i][j];
                    matrix[i][j] = matrix[j][i];
                    matrix[j][i] = v;
                }
            }
        }

        fn flip_lr(matrix: &mut [Vec<i32>]) {
            matrix.iter_mut().for_each(|r| r.reverse());
        }
        transpose(matrix);
        flip_lr(matrix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
        let mut matrix = matrix.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        let expected = [[7, 4, 1], [8, 5, 2], [9, 6, 3]];
        let expected = expected.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
        Solution::rotate(&mut matrix);
        assert_eq!(expected, matrix);
    }
}
