pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut result = 1;
        let mut left = 0;
        let mut max = 0;
        while left < nums.len() {
            let mut curr = nums[left];
            let mut right = left + 1;
            while right < nums.len() && nums[right] >= curr && curr & nums[right] >= nums[right] {
                curr &= nums[right];
                right += 1;
            }
            match curr.cmp(&max) {
                std::cmp::Ordering::Equal => result = result.max(right - left),
                std::cmp::Ordering::Greater => {
                    max = curr;
                    result = right - left;
                }
                _ => {}
            }
            left = right;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_subarray(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            1,
            Solution::longest_subarray(vec![
                96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 96317, 279979
            ])
        );
    }
}
