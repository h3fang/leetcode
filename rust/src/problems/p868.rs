pub struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let mut result = 0;
        n /= 2 * (n & -n);
        while n > 0 {
            let w = (n & -n).trailing_zeros() + 1;
            result = result.max(w);
            n >>= w;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::binary_gap(22));
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::binary_gap(8));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::binary_gap(5));
    }
}
