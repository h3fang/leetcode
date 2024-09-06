pub struct Solution;

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut cache = vec![-1i8; n];
        fn dfs(nums: &[i32], i: usize, cache: &mut [i8]) -> bool {
            if i == nums.len() {
                return true;
            }
            if cache[i] >= 0 {
                return cache[i] == 1;
            }
            if i + 1 < nums.len() && nums[i] == nums[i + 1] && dfs(nums, i + 2, cache) {
                cache[i] = 1;
                return true;
            }

            if i + 2 < nums.len()
                && nums[i] == nums[i + 1]
                && nums[i] == nums[i + 2]
                && dfs(nums, i + 3, cache)
            {
                cache[i] = 1;
                return true;
            }

            if i + 2 < nums.len()
                && nums[i] + 1 == nums[i + 1]
                && nums[i] + 2 == nums[i + 2]
                && dfs(nums, i + 3, cache)
            {
                cache[i] = 1;
                return true;
            }

            cache[i] = 0;
            false
        }
        dfs(&nums, 0, &mut cache)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::valid_partition(vec![4, 4, 4, 5, 6]));
    }

    #[test]
    fn case2() {
        assert!(!Solution::valid_partition(vec![1, 1, 1, 2]));
    }
}
