pub struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut i = 0;
        let mut j = 1;
        let mut result = 1;
        while i < n && j < n {
            let a = nums[j];
            let mut valid = true;
            for k in (i..j).rev() {
                let b = nums[k];
                if a & b != 0 {
                    valid = false;
                    i = k + 1;
                }
            }
            if valid {
                result = result.max(j - i + 1);
                j += 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]));
    }
}
