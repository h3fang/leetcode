pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xor = nums.iter().fold(0, |acc, e| acc ^ *e);
        let last_one = xor & -xor;
        let mut n1 = 0;
        for n in nums {
            if n & last_one == 0 {
                n1 ^= n;
            }
        }
        vec![n1, n1 ^ xor]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut result = Solution::single_number(vec![1, 2, 1, 3, 2, 5]);
        result.sort_unstable();
        assert_eq!(vec![3, 5], result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::single_number(vec![-1, 0]);
        result.sort_unstable();
        assert_eq!(vec![-1, 0], result);
    }
}
