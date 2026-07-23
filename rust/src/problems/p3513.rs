pub struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        match nums.len() {
            n if n <= 2 => n as i32,
            n => 1 << (usize::BITS - n.leading_zeros()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::unique_xor_triplets(vec![1, 2]));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::unique_xor_triplets(vec![3, 1, 2]));
    }

    #[test]
    fn case3() {
        assert_eq!(8, Solution::unique_xor_triplets(vec![3, 1, 2, 4]));
    }
}
