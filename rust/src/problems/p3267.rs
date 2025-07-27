pub struct Solution;

impl Solution {
    pub fn maximum_median_sum(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let n = nums.len() / 3;
        nums[n..].chunks(2).map(|c| c[0] as i64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::maximum_median_sum(vec![2, 1, 3, 2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(20, Solution::maximum_median_sum(vec![1, 1, 10, 10, 10, 10]));
    }
}
