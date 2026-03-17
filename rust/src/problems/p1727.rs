pub struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix[0].len();
        let mut h = vec![0; n];
        let mut idx: Vec<usize> = (0..n).collect();
        let mut non_zero = Vec::with_capacity(n);
        let mut result = 0;
        for row in matrix {
            let mut i = 0;
            for j in 0..n {
                let k = idx[j];
                if row[k] == 0 {
                    h[k] = 0;
                    idx[i] = k;
                    i += 1;
                } else {
                    h[k] += 1;
                    non_zero.push(k);
                }
            }

            for j in non_zero.drain(..) {
                idx[i] = j;
                result = result.max((n - i) * h[j]);
                i += 1;
            }
        }
        result as i32
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
