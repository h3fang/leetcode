pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix[0].len();
        let mut prev = vec![];
        let mut result = 0;
        for row in matrix {
            let mut curr = vec![];
            let mut seen = vec![false; n];
            for (h, col) in prev {
                if row[col] == 1 {
                    curr.push((h + 1, col));
                    seen[col] = true;
                }
            }
            for col in 0..n {
                if !seen[col] && row[col] == 1 {
                    curr.push((1, col));
                }
            }
            curr.iter()
                .enumerate()
                .for_each(|(i, e)| result = result.max((i as i32 + 1) * e.0));
            prev = curr;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [[0, 0, 1], [1, 1, 1], [1, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(4, Solution::largest_submatrix(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[1, 0, 1, 0, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(3, Solution::largest_submatrix(matrix));
    }

    #[test]
    fn case3() {
        let matrix = [[1, 1, 0], [1, 0, 1]].iter().map(|r| r.to_vec()).collect();
        assert_eq!(2, Solution::largest_submatrix(matrix));
    }
}
