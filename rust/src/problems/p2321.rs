pub struct Solution;

impl Solution {
    pub fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        fn max_sub_array(nums1: &[i32], nums2: &[i32]) -> i32 {
            let mut dp = 0;
            let mut result = 0;
            for (a, b) in nums1.iter().zip(nums2) {
                dp = dp.max(0) + b - a;
                result = result.max(dp);
            }
            result
        }
        let m1 = nums1.iter().sum::<i32>();
        let m2 = nums2.iter().sum::<i32>();
        let r1 = m1 + max_sub_array(&nums1, &nums2);
        let r2 = m2 + max_sub_array(&nums2, &nums1);
        r1.max(r2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            210,
            Solution::maximums_spliced_array(vec![60, 60, 60], vec![10, 90, 10])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            220,
            Solution::maximums_spliced_array(vec![20, 40, 20, 70, 30], vec![50, 20, 50, 40, 20])
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            288,
            Solution::maximums_spliced_array(
                vec![28, 34, 38, 14, 30, 31, 23, 7, 28, 3],
                vec![42, 35, 7, 6, 24, 30, 14, 21, 20, 34]
            )
        );
    }
}
