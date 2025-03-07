pub struct Solution;

impl Solution {
    pub fn modified_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for j in 0..matrix[0].len() {
            let max = matrix.iter().map(|r| r[j]).max().unwrap();
            matrix.iter_mut().for_each(|r| {
                if r[j] == -1 {
                    r[j] = max;
                }
            });
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, 2, -1], [4, -1, 6], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        let expected = [[1, 2, 9], [4, 8, 6], [7, 8, 9]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::modified_matrix(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[3, -1], [5, 2]].iter().map(|r| r.to_vec()).collect();
        let expected = [[3, 2], [5, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(expected, Solution::modified_matrix(matrix));
    }
}
