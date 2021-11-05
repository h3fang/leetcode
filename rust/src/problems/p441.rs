pub struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        ((2.0 * n as f64 + 0.25).sqrt() - 0.5) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::arrange_coins(5));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::arrange_coins(8));
    }
}
