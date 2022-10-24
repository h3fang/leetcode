pub struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut left_max = nums[0];
        let mut max = nums[0];
        let mut r = 0;
        for (i, &n) in nums.iter().enumerate().skip(1) {
            max = max.max(n);
            if left_max > n {
                left_max = max;
                r = i;
            }
        }
        r as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::partition_disjoint(vec![5, 0, 3, 8, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]));
    }
}
