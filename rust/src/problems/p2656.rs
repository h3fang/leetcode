pub struct Solution;

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let max = nums.into_iter().max().unwrap();
        (max + max + k - 1) * k / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(18, Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3));
    }

    #[test]
    fn case2() {
        assert_eq!(11, Solution::maximize_sum(vec![5, 5, 5], 2));
    }
}
