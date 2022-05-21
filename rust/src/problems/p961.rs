pub struct Solution;

use rand::prelude::*;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut rng = thread_rng();
        loop {
            let i = rng.gen_range(0..n);
            let mut j = rng.gen_range(0..n);
            while j == i {
                j = rng.gen_range(0..n);
            }
            if nums[i] == nums[j] {
                return nums[i];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::repeated_n_times(vec![1, 2, 3, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::repeated_n_times(vec![2, 1, 2, 5, 3, 2]));
    }
}
