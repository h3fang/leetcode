pub struct Solution;

impl Solution {
    pub fn add(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = a;
            a ^= b;
            b = (temp & b) << 1;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::add(1, 1));
    }
    #[test]
    fn case2() {
        assert_eq!(-1, Solution::add(i32::MIN, i32::MAX));
    }
}
