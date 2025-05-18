pub struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (i, mut x) in nums.into_iter().enumerate().take(28) {
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            if sum == i as i32 {
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
        assert_eq!(2, Solution::smallest_index(vec![1, 3, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::smallest_index(vec![1, 10, 11]));
    }

    #[test]
    fn case3() {
        assert_eq!(-1, Solution::smallest_index(vec![1, 2, 3]));
    }
}
