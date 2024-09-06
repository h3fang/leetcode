pub struct Solution;

impl Solution {
    pub fn find_number_in2_d_array(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let mut i = matrix.len() - 1;
        let mut j = 0;
        loop {
            match matrix[i][j].cmp(&target) {
                std::cmp::Ordering::Less => {
                    j += 1;
                    if j == matrix[0].len() {
                        return false;
                    }
                }
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => {
                    if i == 0 {
                        return false;
                    }
                    i -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert!(Solution::find_number_in2_d_array(matrix, 5));
    }

    #[test]
    fn case2() {
        let matrix = vec![vec![], vec![]];
        assert!(!Solution::find_number_in2_d_array(matrix, 5));
    }

    #[test]
    fn case3() {
        let matrix = vec![];
        assert!(!Solution::find_number_in2_d_array(matrix, 5));
    }
}
