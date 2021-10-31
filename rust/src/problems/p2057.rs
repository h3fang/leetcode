pub struct Solution;

impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (i, n) in nums.iter().take(10).enumerate() {
            if i % 10 == *n as usize {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(0, Solution::smallest_equal(vec![0, 1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::smallest_equal(vec![4, 3, 2, 1]));
    }

    #[test]
    fn case3() {
        assert_eq!(
            -1,
            Solution::smallest_equal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0])
        );
    }
}
