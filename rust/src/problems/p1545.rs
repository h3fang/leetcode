pub struct Solution;

impl Solution {
    pub fn find_kth_bit(_n: i32, k: i32) -> char {
        let r = k & -k;
        let inverted = ((k / r) >> 1) & 1;
        let bit = 1 - (k & 1);
        ((bit ^ inverted) as u8 + b'0') as char
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!('0', Solution::find_kth_bit(3, 1));
    }

    #[test]
    fn case2() {
        assert_eq!('1', Solution::find_kth_bit(4, 11));
    }
}
