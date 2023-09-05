pub struct Solution;

impl Solution {
    pub fn min_number(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        for i in 1..=9 {
            if nums1.contains(&i) && nums2.contains(&i) {
                return i;
            }
        }
        let (a, b) = (
            nums1.into_iter().min().unwrap(),
            nums2.into_iter().min().unwrap(),
        );
        a.min(b) * 10 + a.max(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(15, Solution::min_number(vec![4, 1, 3], vec![5, 7]));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::min_number(vec![3, 5, 2, 6], vec![3, 1, 7]));
    }
}
