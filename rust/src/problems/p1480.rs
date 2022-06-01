pub struct Solution;

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![1, 3, 6, 10], Solution::running_sum(vec![1, 2, 3, 4]));
    }
}
