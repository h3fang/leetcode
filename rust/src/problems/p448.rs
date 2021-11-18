pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let mut j = nums[i] as usize - 1;
            while nums[j] != nums[i] {
                nums.swap(i, j);
                j = nums[i] as usize - 1;
            }
        }

        let mut result = Vec::with_capacity(nums.len());
        for (i, n) in nums.iter().enumerate() {
            if i != *n as usize - 1 {
                result.push(i as i32 + 1);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let mut result = Solution::find_disappeared_numbers(nums);
        result.sort_unstable();
        assert_eq!(vec![5, 6], result);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 1];
        let mut result = Solution::find_disappeared_numbers(nums);
        result.sort_unstable();
        assert_eq!(vec![2], result);
    }
}
