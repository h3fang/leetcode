pub struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let (mut min, mut neg, mut sum) = (i32::MAX, 0, 0);
        for c in matrix.into_iter().flatten() {
            sum += c.abs() as i64;
            min = min.min(c.abs());
            if c < 0 {
                neg += 1;
            }
        }
        if neg % 2 == 0 {
            sum
        } else {
            sum - 2 * min as i64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[1, -1], [-1, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(4, Solution::max_matrix_sum(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[1, 2, 3], [-1, -2, -3], [1, 2, 3]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(16, Solution::max_matrix_sum(matrix));
    }

    #[test]
    fn case3() {
        let matrix = [[-1, 0, -1], [-2, 1, 3], [3, 2, 2]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(15, Solution::max_matrix_sum(matrix));
    }

    #[test]
    fn case4() {
        let matrix = [
            [-10000, -10000, -10000],
            [-10000, -10000, -10000],
            [-10000, -10000, -10000],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(70000, Solution::max_matrix_sum(matrix));
    }
}
