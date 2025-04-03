pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let (mut left, mut right) = (0, m);
        let (mut median1, mut median2) = (0, 0);
        while left <= right {
            let i = (left + right) / 2;
            let j = (m + n).div_ceil(2) - i;
            let ni1 = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let ni = if i == m { i32::MAX } else { nums1[i] };
            let nj1 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let nj = if j == n { i32::MAX } else { nums2[j] };
            if ni1 <= nj {
                median1 = ni1.max(nj1);
                median2 = ni.min(nj);
                left = i + 1;
            } else {
                right = i - 1;
            }
        }
        if (m + n) % 2 == 0 {
            (median1 + median2) as f64 * 0.5
        } else {
            median1 as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-5, "a = {:.5}, b = {:.5}", a, b);
    }

    #[test]
    fn case1() {
        assert_close(
            2.0,
            Solution::find_median_sorted_arrays(vec![1, 3], vec![2]),
        );
    }

    #[test]
    fn case2() {
        assert_close(
            2.5,
            Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]),
        );
    }
}
