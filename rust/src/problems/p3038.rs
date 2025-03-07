pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let score = nums[0] + nums[1];
        nums.chunks_exact(2)
            .skip(1)
            .take_while(|c| c[0] + c[1] == score)
            .count() as i32
            + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::max_operations(vec![3, 2, 1, 4, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_operations(vec![3, 2, 6, 1, 4]));
    }
}
