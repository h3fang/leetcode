pub struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut curr = nums[0];
        for w in nums.windows(2) {
            if w[0] >= w[1] {
                result = result.max(curr);
                curr = w[1];
            } else {
                curr += w[1];
            }
        }
        result.max(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(65, Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]));
    }

    #[test]
    fn case2() {
        assert_eq!(150, Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            33,
            Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12])
        );
    }
}
