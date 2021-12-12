pub struct Solution;

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        nums.sort_unstable_by_key(|e| e.1);
        let mut nums = nums[nums.len() - k as usize..].to_vec();
        nums.sort_unstable_by_key(|e| e.0);
        nums.into_iter().map(|e| e.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(vec![3, 3], Solution::max_subsequence(vec![2, 1, 3, 3], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(
            vec![-1, 3, 4],
            Solution::max_subsequence(vec![-1, -2, 3, 4], 3)
        );
    }
}
