pub struct Solution;

impl Solution {
    pub fn sub_array_ranges(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut result = 0;
        for i in 0..n {
            let mut min = nums[i];
            let mut max = nums[i];
            for &e in &nums[i + 1..] {
                min = min.min(e);
                max = max.max(e);
                result += (max - min) as i64;
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
        assert_eq!(4, Solution::sub_array_ranges(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::sub_array_ranges(vec![1, 3, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(59, Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]));
    }
}
