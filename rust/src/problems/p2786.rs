pub struct Solution;

impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let x = x as i64;
        let (mut even, mut odd) = if nums[0] % 2 == 0 {
            (nums[0] as i64, i64::MIN / 2)
        } else {
            (i64::MIN / 2, nums[0] as i64)
        };
        for &a in &nums[1..] {
            if a % 2 == 0 {
                even = (even + a as i64).max(odd + a as i64 - x);
            } else {
                odd = (odd + a as i64).max(even + a as i64 - x);
            }
        }
        even.max(odd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(13, Solution::max_score(vec![2, 3, 6, 1, 9, 2], 5));
    }

    #[test]
    fn case2() {
        assert_eq!(20, Solution::max_score(vec![2, 4, 6, 8], 3));
    }
}
