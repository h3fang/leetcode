pub struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut result = 0;
        for n in nums {
            sum += n;
            result = result.min(sum);
        }
        -result + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-3, 2, -3, 4, 2];
        assert_eq!(5, Solution::min_start_value(nums));
    }

    #[test]
    fn case2() {
        let nums = vec![1, 2];
        assert_eq!(1, Solution::min_start_value(nums));
    }

    #[test]
    fn case3() {
        let nums = vec![1, -2, -3];
        assert_eq!(5, Solution::min_start_value(nums));
    }
}
