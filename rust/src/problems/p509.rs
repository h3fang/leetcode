pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        (0..n).fold((0, 1), |(a, b), _| (b, a + b)).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::fib(2));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::fib(4));
    }
}
