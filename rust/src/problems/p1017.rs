pub struct Solution;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        let mut v = 0x55555555 ^ (0x55555555 - n);
        let mut s = String::with_capacity(32);
        while v > 0 {
            s.push(if v & 1 > 0 { '1' } else { '0' });
            v >>= 1;
        }
        unsafe { s.as_bytes_mut().reverse() };
        if s.is_empty() { "0".to_string() } else { s }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("110", Solution::base_neg2(2));
    }

    #[test]
    fn case2() {
        assert_eq!("111", Solution::base_neg2(3));
    }

    #[test]
    fn case3() {
        assert_eq!("100", Solution::base_neg2(4));
    }
}
