pub struct Solution;

impl Solution {
    pub fn distinct_integers(n: i32) -> i32 {
        if n > 1 {
            n - 1
        } else {
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::distinct_integers(5));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::distinct_integers(3));
    }
}
