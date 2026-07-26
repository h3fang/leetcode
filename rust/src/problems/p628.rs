pub struct Solution;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let a = nums[n - 1] * nums[n - 2] * nums[n - 3];
        let b = nums[n - 1] * nums[0] * nums[1];
        a.max(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::maximum_product(vec![1, 2, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(24, Solution::maximum_product(vec![1, 2, 3, 4]));
    }

    #[test]
    fn case3() {
        assert_eq!(-6, Solution::maximum_product(vec![-1, -2, -3]));
    }
}
