pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let (mut robbed, mut idle) = (nums[0], 0);
        for n in nums.into_iter().skip(1) {
            let robbed_new = idle + n;
            idle = idle.max(robbed);
            robbed = robbed_new;
        }

        idle.max(robbed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
