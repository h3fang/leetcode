pub struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let (m, n) = (mat.len(), mat[0].len());
        let mut presum = vec![vec![0; n + 1]; m + 1];
        for (i, r) in mat.iter().enumerate() {
            for (j, &x) in r.iter().enumerate() {
                presum[i + 1][j + 1] = x + presum[i + 1][j] + presum[i][j + 1] - presum[i][j];
            }
        }

        let mut k = 0;
        for i in 0..m {
            for j in 0..n {
                while i >= k
                    && j >= k
                    && presum[i + 1][j + 1] - presum[i + 1][j - k] - presum[i - k][j + 1]
                        + presum[i - k][j - k]
                        <= threshold
                {
                    k += 1;
                }
            }
        }
        k as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mat = [
            [1, 1, 3, 2, 4, 3, 2],
            [1, 1, 3, 2, 4, 3, 2],
            [1, 1, 3, 2, 4, 3, 2],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(2, Solution::max_side_length(mat, 4));
    }

    #[test]
    fn case2() {
        let mat = [
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
            [2, 2, 2, 2, 2],
        ]
        .iter()
        .map(|r| r.to_vec())
        .collect();
        assert_eq!(0, Solution::max_side_length(mat, 1));
    }
}
