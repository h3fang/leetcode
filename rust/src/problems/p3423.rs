pub struct Solution;

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        let max = nums.windows(2).map(|w| (w[0] - w[1]).abs()).max().unwrap();
        (nums[0] - *nums.last().unwrap()).abs().max(max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_adjacent_distance(vec![1, 2, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::max_adjacent_distance(vec![-5, -10, -5]));
    }
}
