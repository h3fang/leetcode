pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut result = 0;
        let (mut min, mut max, mut left) = (-1, -1, -1);
        for (i, &n) in nums.iter().enumerate() {
            if n < min_k || n > max_k {
                left = i as i32;
            }
            if n == min_k {
                min = i as i32;
            }
            if n == max_k {
                max = i as i32;
            }
            result += (max.min(min) - left).max(0) as i64;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_subarrays(vec![1, 3, 5, 2, 7, 5], 1, 5));
    }

    #[test]
    fn case2() {
        assert_eq!(10, Solution::count_subarrays(vec![1, 1, 1, 1], 1, 1));
    }
}
