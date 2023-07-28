pub struct Solution;

impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut f = nums.to_vec();
        for i in (0..n - 1).rev() {
            for j in i + 1..n {
                f[j] = (nums[i] - f[j]).max(nums[j] - f[j - 1]);
            }
        }
        f[n - 1] >= 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::predict_the_winner(vec![1, 5, 2]));
    }

    #[test]
    fn case2() {
        assert!(Solution::predict_the_winner(vec![1, 5, 233, 7]));
    }
}
