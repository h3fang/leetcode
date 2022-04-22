pub struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut dp = nums.iter().enumerate().fold(0, |acc, (i, &e)| {
            sum += e;
            acc + i as i32 * e
        });
        let n = nums.len() as i32;
        let mut max = dp;
        for e in nums.iter().skip(1).rev() {
            dp += sum - e * n;
            max = max.max(dp);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(26, Solution::max_rotate_function(vec![4, 3, 2, 6]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::max_rotate_function(vec![100]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            330,
            Solution::max_rotate_function(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
        );
    }
}
