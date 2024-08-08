pub struct Solution;

impl Solution {
    pub fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut f = [1; 2];
        let mut result = 1;
        for (i, (&a, &b)) in nums1.iter().zip(&nums2).enumerate().skip(1) {
            let mut g = [1; 2];
            let min = a.min(b);
            if min >= nums1[i - 1].min(nums2[i - 1]) {
                g[0] = f[0] + 1;
            }
            if min >= nums1[i - 1].max(nums2[i - 1]) {
                g[0] = g[0].max(f[1] + 1);
            }
            let max = a.max(b);
            if max >= nums1[i - 1].min(nums2[i - 1]) {
                g[1] = f[0] + 1;
            }
            if max >= nums1[i - 1].max(nums2[i - 1]) {
                g[1] = g[1].max(f[1] + 1);
            }
            result = result.max(g[0].max(g[1]));
            f = g;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            2,
            Solution::max_non_decreasing_length(vec![2, 3, 1], vec![1, 2, 1])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            4,
            Solution::max_non_decreasing_length(vec![1, 3, 2, 1], vec![2, 2, 3, 4])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            2,
            Solution::max_non_decreasing_length(vec![1, 1], vec![2, 2])
        );
    }
}
