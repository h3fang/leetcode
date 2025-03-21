pub struct Solution;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let mut result = Vec::with_capacity(nums1.len().min(nums2.len()));
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len() {
            match nums1[i].cmp(&nums2[j]) {
                std::cmp::Ordering::Less => i += 1,
                std::cmp::Ordering::Equal => {
                    result.push(nums1[i]);
                    i += 1;
                    j += 1;
                }
                std::cmp::Ordering::Greater => j += 1,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]);
        result.sort_unstable();
        let mut expected = vec![2, 2];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        result.sort_unstable();
        let mut expected = vec![4, 9];
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
