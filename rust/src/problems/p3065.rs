pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().filter(|&x| x < k).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_operations(vec![2, 11, 10, 1, 3], 10));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::min_operations(vec![1, 1, 2, 4, 9], 1));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::min_operations(vec![1, 1, 2, 4, 9], 9));
    }
}
