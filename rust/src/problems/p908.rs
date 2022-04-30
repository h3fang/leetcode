pub struct Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = nums[0];
        let mut max = min;
        nums.iter().for_each(|&n| {
            min = min.min(n);
            max = max.max(n);
        });
        (max - min - 2 * k).max(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::smallest_range_i(vec![1], 0));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::smallest_range_i(vec![0, 10], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::smallest_range_i(vec![1, 3, 6], 3));
    }
}
