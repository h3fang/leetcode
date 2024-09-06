pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let a = n ^ (n >> 1);
        a & (a + 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::has_alternating_bits(5));
    }

    #[test]
    fn case2() {
        assert!(!Solution::has_alternating_bits(7));
    }
}
