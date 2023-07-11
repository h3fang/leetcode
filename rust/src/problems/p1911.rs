pub struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        (1..nums.len()).fold(nums[0] as i64, |r, i| {
            r + (0.max(nums[i] - nums[i - 1]) as i64)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::max_alternating_sum(vec![4, 2, 5, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::max_alternating_sum(vec![5, 6, 7, 8]));
    }

    #[test]
    fn case3() {
        assert_eq!(10, Solution::max_alternating_sum(vec![6, 2, 1, 2, 4, 5]));
    }
}
