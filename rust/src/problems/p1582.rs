pub struct Solution;

impl Solution {
    pub fn num_special(mut mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        #[allow(clippy::needless_range_loop)]
        for i in 0..m {
            let sum = mat[i].iter().sum::<i32>() - i32::from(i == 0);
            if sum > 0 {
                for j in 0..n {
                    if mat[i][j] == 1 {
                        mat[0][j] += sum;
                    }
                }
            }
        }
        mat[0].iter().filter(|&&e| e == 1).count() as i32
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
