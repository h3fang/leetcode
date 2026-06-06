pub struct Solution;

impl Solution {
    pub fn left_right_difference(mut nums: Vec<i32>) -> Vec<i32> {
        let mut right: i32 = nums.iter().sum();
        let mut left = 0;
        for x in nums.iter_mut() {
            let t = *x;
            right -= t;
            *x = (left - right).abs();
            left += t;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            vec![15, 1, 11, 22],
            Solution::left_right_difference(vec![10, 4, 8, 3])
        );
    }

    #[test]
    fn case2() {
        assert_eq!(vec![0], Solution::left_right_difference(vec![1]));
    }
}
