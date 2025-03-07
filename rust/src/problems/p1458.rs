pub struct Solution;

impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (m, n) = (nums1.len(), nums2.len());
        let mut f = vec![vec![i32::MIN / 2; n + 1]; m + 1];
        for (i, &a) in nums1.iter().enumerate() {
            for (j, &b) in nums2.iter().enumerate() {
                let c = a * b;
                f[i + 1][j + 1] = c.max(f[i][j + 1]).max(f[i + 1][j]).max(f[i][j] + c);
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
            18,
            Solution::max_dot_product(vec![2, 1, -2, 5], vec![3, 0, -6])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(21, Solution::max_dot_product(vec![3, -2], vec![2, -6, 7]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::max_dot_product(vec![-1, -1], vec![1, 1]));
    }
}
