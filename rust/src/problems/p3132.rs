pub struct Solution;

impl Solution {
    pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        let n = nums2.len();
        nums1.sort_unstable();
        nums2.sort_unstable();
        for i in [2, 1] {
            let x = nums2[0] - nums1[i];
            let mut j = 0;
            for &a in &nums1[i..] {
                if a + x == nums2[j] {
                    j += 1;
                }
                if j == n {
                    return x;
                }
            }
        }
        nums2[0] - nums1[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            -2,
            Solution::minimum_added_integer(vec![4, 20, 16, 12, 8], vec![14, 18, 10])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            2,
            Solution::minimum_added_integer(vec![3, 5, 5, 3], vec![7, 7])
        );
    }
}
