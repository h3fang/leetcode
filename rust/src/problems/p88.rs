pub struct Solution;

impl Solution {
    #[allow(clippy::ptr_arg)]
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m - 1;
        let mut p2 = n - 1;
        let mut last = m + n - 1;
        while p1 >= 0 || p2 >= 0 {
            nums1[last as usize] = if p1 < 0 {
                p2 -= 1;
                nums2[(p2 + 1) as usize]
            } else if p2 < 0 {
                p1 -= 1;
                nums1[(p1 + 1) as usize]
            } else if nums1[p1 as usize] < nums2[p2 as usize] {
                p2 -= 1;
                nums2[(p2 + 1) as usize]
            } else {
                p1 -= 1;
                nums1[(p1 + 1) as usize]
            };
            last -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }
}
