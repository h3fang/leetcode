pub struct Solution;

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len() - 1;
        (nums[n] - 1) * (nums[n - 1] - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::max_product(vec![3, 4, 5, 2]));
    }
}
