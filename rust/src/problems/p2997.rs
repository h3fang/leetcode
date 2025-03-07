pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().fold(k, |acc, x| acc ^ x).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_operations(vec![2, 1, 3, 4], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_operations(vec![2, 0, 2, 0], 0));
    }
}
