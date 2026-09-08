pub struct Solution;

impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        0.max(n - 999)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::count_commas(1002));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_commas(998));
    }
}
