pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total: i32 = nums.iter().sum();
        let mut left = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if 2 * left + n == total {
                return i as i32;
            }
            left += n;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
    }
}
