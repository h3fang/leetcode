pub struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut left = *nums.iter().max().unwrap();
        let mut right = nums.iter().sum();
        let mut result = 0;
        while left <= right {
            let mid = (left + right) / 2;
            let mut k = 1;
            let mut sum = 0;
            for &n in &nums {
                sum += n;
                if sum > mid {
                    k += 1;
                    sum = n;
                }
            }
            if k <= m {
                right = mid - 1;
                result = mid;
            } else {
                left = mid + 1;
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
        assert_eq!(18, Solution::split_array(vec![7, 2, 5, 10, 8], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(9, Solution::split_array(vec![1, 2, 3, 4, 5], 2));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::split_array(vec![1, 4, 4], 3));
    }
}
