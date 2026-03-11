pub struct Solution;

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mask = (1 << (32 - n.leading_zeros())) - 1;
        n ^ mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::bitwise_complement(0));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::bitwise_complement(5));
    }
}
