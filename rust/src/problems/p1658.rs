pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total = nums.iter().sum::<i32>();
        if total < x {
            return -1;
        }
        let n = nums.len();
        let target = total - x;
        let mut left = 0;
        let mut sum = 0;
        let mut max = -1;
        for right in 0..n {
            sum += nums[right];
            while sum > target {
                sum -= nums[left];
                left += 1;
            }
            if sum == target {
                max = max.max(right as i32 - left as i32 + 1);
            }
        }
        if -1 == max {
            -1
        } else {
            n as i32 - max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_operations(vec![1, 1, 4, 2, 3], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::min_operations(vec![5, 6, 7, 8, 9], 4));
    }

    #[test]
    fn case3() {
        assert_eq!(
            16,
            Solution::min_operations(
                vec![
                    8828, 9581, 49, 9818, 9974, 9869, 9991, 10000, 10000, 10000, 9999, 9993, 9904,
                    8819, 1231, 6309
                ],
                134365
            )
        );
    }
}
