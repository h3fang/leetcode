pub struct Solution;

impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        fn calc(slices: &[i32]) -> i32 {
            let m = slices.len();
            let n = (m + 1) / 3;
            let mut f = vec![vec![i32::MIN; n + 1]; m];
            f[0][0] = 0;
            f[0][1] = slices[0];
            f[1][0] = 0;
            f[1][1] = slices[0].max(slices[1]);
            for i in 2..m {
                f[i][0] = 0;
                for j in 1..=n {
                    f[i][j] = f[i - 1][j].max(f[i - 2][j - 1] + slices[i]);
                }
            }
            f[m - 1][n]
        }
        calc(&slices[1..]).max(calc(&slices[..slices.len() - 1]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::max_size_slices(vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::max_size_slices(vec![8, 9, 8, 6, 1, 1]));
    }
}
