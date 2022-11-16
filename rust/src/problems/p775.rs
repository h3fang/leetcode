pub struct Solution;

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n < 3 {
            return true;
        }
        let mut min = nums[n - 1];
        for i in (0..n - 2).rev() {
            if nums[i] > min {
                return false;
            }
            min = min.min(nums[i + 1]);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::is_ideal_permutation(vec![1, 0, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::is_ideal_permutation(vec![1, 2, 0]));
    }
}
