pub struct Solution;

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut count = 0;
        for &n in nums.iter().chain(&[1]) {
            if n == 0 {
                count += 1;
            } else {
                result += (count + 1) * count / 2;
                count = 0;
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
        assert_eq!(
            6,
            Solution::zero_filled_subarray(vec![1, 3, 0, 0, 2, 0, 0, 4])
        );
    }
}
