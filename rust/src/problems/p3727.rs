pub struct Solution;

impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable_by_key(|x| -x.abs());
        let n = nums.len().div_ceil(2);
        let mut ans = nums[..n].iter().map(|&x| (x * x) as i64).sum();
        for x in &nums[n..] {
            ans -= (x * x) as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(12, Solution::max_alternating_sum(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(16, Solution::max_alternating_sum(vec![1, -1, 2, -2, 3, -3]));
    }
}
