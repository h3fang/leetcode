pub struct Solution;

impl Solution {
    pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
        let (mut fixed, mut pre) = (0, 0);
        for &x in &nums {
            fixed |= pre & x as i64;
            pre |= x as i64;
        }
        let mut result = 0;
        for x in nums {
            let x = x as i64;
            result = result.max((pre ^ x) | fixed | (x << k));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(30, Solution::maximum_or(vec![12, 9], 1));
    }

    #[test]
    fn case2() {
        assert_eq!(35, Solution::maximum_or(vec![8, 1, 2], 2));
    }
}
