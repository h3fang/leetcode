pub struct Solution;

impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let (n, mut i, mut result) = (nums.len(), 0, -1);
        while i < n {
            if i > 0 && nums[i] == nums[i - 1] + 1 {
                let mut j = 1;
                while i + j < n && nums[i - 1 + (j - 1) % 2] == nums[i + j] {
                    j += 1;
                }
                result = result.max(j as i32 + 1);
                i += j;
            } else {
                i += 1;
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
        assert_eq!(4, Solution::alternating_subarray(vec![2, 3, 4, 3, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::alternating_subarray(vec![4, 5, 6]));
    }
}
