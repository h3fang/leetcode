pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let mut set = HashSet::new();
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut num = 0;
        if n < k {
            return false;
        }
        for b in &s[..k] {
            num = (num << 1) + (b - b'0') as i32;
        }
        set.insert(num);
        let mask = (1 << k) - 1;
        for b in &s[k..n] {
            num = (num << 1) + (b - b'0') as i32;
            set.insert(num & mask);
        }
        set.len() == mask as usize + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::has_all_codes("00110110".into(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::has_all_codes("0110".into(), 2));
    }
}
