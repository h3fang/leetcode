pub struct Solution;

impl Solution {
    pub fn gcd_of_odd_even_sums(n: i32) -> i32 {
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::gcd_of_odd_even_sums(4));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::gcd_of_odd_even_sums(5));
    }
}
