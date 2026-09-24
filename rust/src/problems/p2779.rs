pub struct Solution;

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (mut result, mut l) = (0, 0);
        for (r, x) in nums.iter().enumerate() {
            while x - nums[l] > 2 * k {
                l += 1;
            }
            result = result.max(r - l + 1);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::maximum_beauty(vec![4, 6, 1, 2], 2));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::maximum_beauty(vec![1, 1, 1, 1], 10));
    }
}
