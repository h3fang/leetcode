pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut k = 0;
        let mut result = Vec::with_capacity(m * n);
        loop {
            let p = k / 4;
            match k % 4 {
                0 => {
                    if p == n - p {
                        break;
                    }
                    result.extend(&matrix[p][p..n - p]);
                }
                1 => {
                    if p + 1 == m - p {
                        break;
                    }
                    for r in &matrix[p + 1..m - p] {
                        result.push(r[n - p - 1]);
                    }
                }
                2 => {
                    if p == n - p - 1 {
                        break;
                    }
                    result.extend(matrix[m - 1 - p][p..n - p - 1].iter().rev());
                }
                _ => {
                    if p + 1 == m - p - 1 {
                        break;
                    }
                    for i in (p + 1..m - p - 1).rev() {
                        result.push(matrix[i][p]);
                    }
                }
            }
            k += 1;
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
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(expected, Solution::spiral_order(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
        let matrix = matrix.iter().map(|r| r.to_vec()).collect();
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(expected, Solution::spiral_order(matrix));
    }
}
