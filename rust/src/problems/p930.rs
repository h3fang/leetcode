pub struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let (mut sum1, mut sum2, mut l1, mut l2, mut result) = (0, 0, 0, 0, 0);
        for (r, &x) in nums.iter().enumerate() {
            sum1 += x;
            while l1 <= r && sum1 > goal {
                sum1 -= nums[l1];
                l1 += 1;
            }
            sum2 += x;
            while l2 <= r && sum2 >= goal {
                sum2 -= nums[l2];
                l2 += 1;
            }
            result += l2 - l1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::num_subarrays_with_sum(vec![0, 0, 0, 0, 0], 0));
    }
}
