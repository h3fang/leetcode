pub struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        (0..n / 2).map(|i| nums[i] + nums[n - i - 1]).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::min_pair_sum(vec![3, 5, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]));
    }
}
