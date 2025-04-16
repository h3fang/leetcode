pub struct Solution;

impl Solution {
    pub fn minimum_one_bit_operations(mut n: i32) -> i32 {
        let mut result = 0;
        while n > 0 {
            result ^= n;
            n >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_one_bit_operations(3));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::minimum_one_bit_operations(6));
    }
}
