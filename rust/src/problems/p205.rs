pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut m: HashMap<u8, u8> = HashMap::new();
        let mut mapped = [false; 256];
        let s = s.as_bytes();
        let t = t.as_bytes();
        for (&a, &b) in s.iter().zip(t) {
            if let Some(&c) = m.get(&a) {
                if c != b {
                    return false;
                }
            } else {
                if mapped[b as usize] {
                    return false;
                }
                mapped[b as usize] = true;
                m.insert(a, b);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_isomorphic("egg".into(), "add".into()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_isomorphic("egg".into(), "ada".into()));
    }
}
