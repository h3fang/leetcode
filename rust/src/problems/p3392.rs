pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.windows(3)
            .filter(|w| (w[0] + w[2]) * 2 == w[1])
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::count_subarrays(vec![1, 2, 1, 4, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_subarrays(vec![1, 1, 1]));
    }
}
