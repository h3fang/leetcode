pub struct Solution;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let (n, k) = (s.len(), k as usize);
        let mask = (1 << k) - 1;
        if n < mask + k {
            return false;
        }

        let mut set = vec![false; mask + 1];
        let s = s.as_bytes();
        let (mut num, mut count) = (0, 1);
        for b in &s[..k] {
            num = (num << 1) + (b - b'0') as usize;
        }
        set[num] = true;

        for b in &s[k..n] {
            num = (num << 1) + (b - b'0') as usize;
            let idx = num & mask;
            if !set[idx] {
                set[idx] = true;
                count += 1;
            }
            if count == mask + 1 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::has_all_codes("00110110".into(), 2));
    }

    #[test]
    fn case2() {
        assert!(!Solution::has_all_codes("0110".into(), 2));
    }
}
