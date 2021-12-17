pub struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();

        let mut result = 0;
        let mut dp = vec![0; n + 1];
        let mut prev = 0;
        for i in 1..=m {
            for j in 1..=n {
                let temp = dp[j];
                if matrix[i - 1][j - 1] == '1' {
                    dp[j] = dp[j].min(prev.min(dp[j - 1])) + 1;
                    result = result.max(dp[j]);
                } else {
                    dp[j] = 0;
                }
                prev = temp;
            }
        }

        result * result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ];
        let matrix = matrix
            .iter()
            .map(|row| row.iter().map(|c| c.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(4, Solution::maximal_square(matrix));
    }

    #[test]
    fn case2() {
        let matrix = [
            ["1", "1", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["0", "0", "0", "0", "0"],
            ["1", "1", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
        ];
        let matrix = matrix
            .iter()
            .map(|row| row.iter().map(|c| c.as_bytes()[0] as char).collect())
            .collect();
        assert_eq!(4, Solution::maximal_square(matrix));
    }
}
