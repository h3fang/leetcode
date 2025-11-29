pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().sum::<i32>() % k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::min_operations(vec![3, 9, 7], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_operations(vec![4, 1, 3], 4));
    }

    #[test]
    fn case3() {
        assert_eq!(5, Solution::min_operations(vec![3, 2], 6));
    }
}
