pub struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let n = n as i64;
        2 * n * (n - 1) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::colored_cells(1));
    }

    #[test]
    fn case2() {
        assert_eq!(5, Solution::colored_cells(2));
    }
}
