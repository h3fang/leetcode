pub struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut result = 0;
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 1
                    && (0..m).all(|k| k == i || mat[k][j] == 0)
                    && (0..n).all(|k| k == j || mat[i][k] == 0)
                {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [[1, 0, 0], [0, 0, 1], [1, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(1, Solution::num_special(mat));
    }

    #[test]
    fn case2() {
        let mat = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(3, Solution::num_special(mat));
    }

    #[test]
    fn case3() {
        let mat = [[0, 0, 0, 1], [1, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]]
            .iter()
            .map(|r| r.to_vec())
            .collect();
        assert_eq!(2, Solution::num_special(mat));
    }
}
