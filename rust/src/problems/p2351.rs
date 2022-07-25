pub struct Solution;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut mask = 0u32;
        for &c in s.as_bytes() {
            let i = (c - b'a') as usize;
            if mask & (1 << i) > 0 {
                return c as char;
            }
            mask |= 1 << i;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!('c', Solution::repeated_character("abccbaacz".into()));
    }
}
