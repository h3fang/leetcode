pub struct Solution;

impl Solution {
    pub fn sum_nums(mut n: i32) -> i32 {
        #[allow(clippy::unit_cmp)]
        let _ = n > 1 && (n += Self::sum_nums(n - 1)) == ();
        n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(6, Solution::sum_nums(3));
    }

    #[test]
    fn case2() {
        assert_eq!(50005000, Solution::sum_nums(10000));
    }
}
