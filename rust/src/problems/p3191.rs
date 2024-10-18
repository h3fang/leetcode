pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        for i in 0..n - 2 {
            if nums[i] == 0 {
                result += 1;
                nums[i + 1] = 1 - nums[i + 1];
                nums[i + 2] = 1 - nums[i + 2];
            }
        }
        if nums[n - 2] != 1 || nums[n - 1] != 1 {
            -1
        } else {
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::min_operations(vec![0, 1, 1, 1, 0, 0]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_operations(vec![0, 1, 1, 1]));
    }
}
