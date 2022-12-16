pub struct Solution;

impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let sum = nums.iter().map(|&n| n as i64).sum::<i64>();
        let goal = (goal as i64 - sum).abs();
        ((goal + limit as i64 - 1) / limit as i64) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_elements(vec![1, -1, 1], 3, -4));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_elements(vec![1, -10, 9, 1], 100, 0));
    }
}
