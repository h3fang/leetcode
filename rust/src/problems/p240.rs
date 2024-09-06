pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut i = m - 1;
        let mut j = 0;
        while j < n {
            match matrix[i][j].cmp(&target) {
                std::cmp::Ordering::Less => j += 1,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => {
                    if i == 0 {
                        break;
                    } else {
                        i -= 1;
                    }
                }
            }
        }
        false
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
        let target = 5;
        assert!(Solution::search_matrix(matrix, target));
    }

    #[test]
    fn case2() {
        let matrix = vec![vec![7]];
        let target = 7;
        assert!(Solution::search_matrix(matrix, target));
    }

    #[test]
    fn case3() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        let target = 20;
        assert!(!Solution::search_matrix(matrix, target));
    }

    #[test]
    fn case4() {
        let matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let target = 5;
        assert!(Solution::search_matrix(matrix, target));
    }
}
