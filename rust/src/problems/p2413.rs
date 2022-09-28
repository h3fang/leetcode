pub struct Solution;

impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 {
            n
        } else {
            2 * n
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(10, Solution::smallest_even_multiple(5));
    }

    #[test]
    fn case2() {
        assert_eq!(6, Solution::smallest_even_multiple(6));
    }
}
