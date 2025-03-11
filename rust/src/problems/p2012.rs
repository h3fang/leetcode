pub struct Solution;

impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut left_max = vec![0; n];
        left_max[0] = nums[0];
        for (i, &x) in nums.iter().enumerate().skip(1) {
            left_max[i] = left_max[i - 1].max(x);
        }
        let mut ans = 0;
        let mut right_min = nums[n - 1];
        for (i, &x) in nums.iter().enumerate().rev().skip(1).take(n - 2) {
            if x > left_max[i - 1] && x < right_min {
                ans += 2;
            } else if x > nums[i - 1] && x < nums[i + 1] {
                ans += 1;
            }
            right_min = right_min.min(x);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::sum_of_beauties(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::sum_of_beauties(vec![2, 4, 6, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::sum_of_beauties(vec![3, 2, 1]));
    }
}
