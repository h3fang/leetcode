pub struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let k = i32::BITS - n.leading_zeros();
        (1 << k) - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(7, Solution::smallest_number(5));
    }

    #[test]
    fn case2() {
        assert_eq!(15, Solution::smallest_number(10));
    }

    #[test]
    fn case3() {
        assert_eq!(3, Solution::smallest_number(3));
    }
}
