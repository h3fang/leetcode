pub struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut s = Vec::with_capacity(nums.len());
        for (i, &x) in nums.iter().enumerate() {
            if s.is_empty() || nums[*s.last().unwrap()] > x {
                s.push(i);
            }
        }
        let mut result = 0;
        for (j, &x) in nums.iter().enumerate().rev() {
            while !s.is_empty() && x >= nums[*s.last().unwrap()] {
                result = result.max(j - *s.last().unwrap());
                s.pop();
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
        assert_eq!(4, Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]));
    }

    #[test]
    fn case2() {
        assert_eq!(
            7,
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1])
        );
    }
}
