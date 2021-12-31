pub struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut r = 0;
        for i in 1..k + 1 {
            r = (r * 10 + 1) % k;
            if r == 0 {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(1, Solution::smallest_repunit_div_by_k(1));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(2));
    }
}
