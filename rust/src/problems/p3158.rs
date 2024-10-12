pub struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut m = 0u64;
        let mut result = 0;
        for x in nums {
            let bit = 1 << x;
            if m & bit > 0 {
                result ^= x;
            }
            m |= bit;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::duplicate_numbers_xor(vec![1, 2, 1, 3]));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::duplicate_numbers_xor(vec![1, 2, 3]));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::duplicate_numbers_xor(vec![1, 2, 2, 1]));
    }
}
