pub struct Solution;

impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let m = nums1.len();
        let n = nums2.len();
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if nums1[i] == nums2[j] {
                    dp[i][j] = dp[i + 1][j + 1] + 1;
                }
            }
        }
        *dp.iter().flatten().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0])
        );
    }
}
