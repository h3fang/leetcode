pub struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        fn f(nums: &[i32], score: i32) -> i32 {
            let n = nums.len();
            let mut cache = vec![vec![-1; n]; n];
            fn dfs(nums: &[i32], score: i32, i: i32, j: i32, cache: &mut [Vec<i32>]) -> i32 {
                if i >= j {
                    return 0;
                }
                if cache[i as usize][j as usize] != -1 {
                    return cache[i as usize][j as usize];
                }
                let mut result = 0;
                if nums[i as usize] + nums[i as usize + 1] == score {
                    result = result.max(1 + dfs(nums, score, i + 2, j, cache));
                }
                if nums[j as usize - 1] + nums[j as usize] == score {
                    result = result.max(1 + dfs(nums, score, i, j - 2, cache));
                }
                if nums[i as usize] + nums[j as usize] == score {
                    result = result.max(1 + dfs(nums, score, i + 1, j - 1, cache));
                }
                cache[i as usize][j as usize] = result;
                result
            }
            dfs(nums, score, 0, nums.len() as i32 - 1, &mut cache)
        }
        let n = nums.len();
        f(&nums, nums[0] + nums[1])
            .max(f(&nums, nums[0] + nums[n - 1]).max(f(&nums, nums[n - 2] + nums[n - 1])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_operations(vec![3, 2, 1, 2, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_operations(vec![3, 2, 6, 1, 4]));
    }
}
