pub struct Solution;

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let min1 = nums1.into_iter().min().unwrap();
        let min2 = nums2.into_iter().min().unwrap();
        min2 - min1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::added_integer(vec![2, 6, 4], vec![9, 7, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(-5, Solution::added_integer(vec![10], vec![5]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1])
        );
    }
}
