pub struct Solution;

impl Solution {
    pub fn range_bitwise_and(mut left: i32, mut right: i32) -> i32 {
        let mut i = 0;

        while left != right {
            left >>= 1;
            right >>= 1;
            i += 1;
        }

        left << i
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::range_bitwise_and(5, 7));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::range_bitwise_and(0, 0));
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::range_bitwise_and(1, 2147483647));
    }

    #[test]
    fn case4() {
        assert_eq!(0, Solution::range_bitwise_and(1, 7));
    }

    #[test]
    fn case5() {
        assert_eq!(0, Solution::range_bitwise_and(0, 2));
    }
}
