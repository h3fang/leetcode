pub struct Solution;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => return nums1[i],
                std::cmp::Ordering::Greater => j += 1,
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3], vec![2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::get_common(vec![1, 2, 3, 6], vec![2, 3, 4, 5]));
    }
}
