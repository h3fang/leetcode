pub struct Solution;

impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut result = 0;
        let mut last = *nums.last().unwrap();
        for (i, n) in nums.into_iter().rev().enumerate().skip(1) {
            if n != last {
                result += i as i32;
                last = n;
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
        assert_eq!(3, Solution::reduction_operations(vec![5, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::reduction_operations(vec![1, 1, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(4, Solution::reduction_operations(vec![1, 1, 2, 2, 3]));
    }
}
