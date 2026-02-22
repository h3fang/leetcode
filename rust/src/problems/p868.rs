pub struct Solution;

impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let (mut result, mut last) = (0, 32);
        while n > 0 {
            let zeros = n.trailing_zeros() as i32;
            result = result.max(zeros - last);
            last = zeros;
            n &= n - 1;
        }
        result
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
