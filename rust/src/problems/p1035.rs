pub struct Solution;

impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut f = vec![vec![0; n + 1]; m + 1];
        for (i, &a) in nums1.iter().enumerate() {
            for (j, &b) in nums2.iter().enumerate() {
                if a == b {
                    f[i + 1][j + 1] = 1 + f[i][j];
                } else {
                    f[i + 1][j + 1] = f[i + 1][j].max(f[i][j + 1]);
                }
            }
        }
        f[m][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            3,
            Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1])
        );
    }
}
