pub struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut result = -1;
        for n in nums {
            if n <= min {
                min = n;
            } else {
                result = result.max(n - min);
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
        assert_eq!(4, Solution::maximum_difference(vec![7, 1, 5, 4]));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::maximum_difference(vec![9, 4, 3, 2]));
    }
}
