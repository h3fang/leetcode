pub struct Solution;

impl Solution {
    pub fn reconstruct_matrix(mut upper: i32, mut lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; colsum.len()]; 2];
        for (i, c) in colsum.into_iter().enumerate() {
            match c {
                1 => {
                    if upper > lower {
                        result[0][i] = 1;
                        upper -= 1;
                    } else {
                        result[1][i] = 1;
                        lower -= 1;
                    }
                }
                2 => {
                    result[0][i] = 1;
                    result[1][i] = 1;
                    upper -= 1;
                    lower -= 1;
                }
                _ => {}
            }
        }
        if upper != 0 || lower != 0 {
            return vec![];
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(m: &[Vec<i32>], upper: i32, lower: i32, colsum: &[i32]) {
        println!("{m:?}");
        assert!(m.len() == 2 && m[0].len() == colsum.len());
        let mut u = 0;
        let mut l = 0;
        for (i, &c) in colsum.iter().enumerate() {
            u += m[0][i];
            l += m[1][i];
            assert_eq!(c, m[0][i] + m[1][i]);
        }
        assert_eq!(upper, u);
        assert_eq!(lower, l);
    }

    #[test]
    fn case1() {
        let upper = 2;
        let lower = 1;
        let colsum = vec![1, 1, 1];
        let result = Solution::reconstruct_matrix(upper, lower, colsum.to_vec());
        check(&result, upper, lower, &colsum);
    }

    #[test]
    fn case2() {
        let upper = 2;
        let lower = 3;
        let colsum = [2, 2, 1, 1];
        let result = Solution::reconstruct_matrix(upper, lower, colsum.to_vec());
        assert!(result.is_empty());
    }

    #[test]
    fn case3() {
        let upper = 5;
        let lower = 5;
        let colsum = vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1];
        let result = Solution::reconstruct_matrix(upper, lower, colsum.to_vec());
        check(&result, upper, lower, &colsum);
    }
}
