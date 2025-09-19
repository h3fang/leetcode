pub struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        if n1.is_multiple_of(2) && n2.is_multiple_of(2) {
            0
        } else {
            let mut result = 0;
            if n2 % 2 == 1 {
                result = nums1.iter().fold(0, |acc, n| acc ^ *n);
            }
            if n1 % 2 == 1 {
                result = nums2.iter().fold(result, |acc, n| acc ^ *n);
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::xor_all_nums(vec![2, 1, 3], vec![10, 2, 5, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::xor_all_nums(vec![1, 2], vec![3, 4]));
    }
}
