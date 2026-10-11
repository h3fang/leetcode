pub struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.into_iter()
            .enumerate()
            .map(|(i, x)| if n.is_multiple_of(i + 1) { x * x } else { 0 })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(21, Solution::sum_of_squares(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(63, Solution::sum_of_squares(vec![2, 7, 1, 19, 18, 3]));
    }
}
