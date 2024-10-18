pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let (mut f, mut result) = (0, 0);
        for x in nums {
            if x ^ f == 0 {
                f ^= 1;
                result += 1;
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
        assert_eq!(4, Solution::min_operations(vec![0, 1, 1, 0, 1]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::min_operations(vec![1, 0, 0, 0]));
    }
}
