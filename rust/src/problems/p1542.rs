pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        let mut m = HashMap::with_capacity(1024);
        m.insert(0, -1);
        let mut result = 0;
        let mut t = 0;
        for (j, &b) in s.as_bytes().iter().enumerate() {
            let j = j as i32;
            t ^= 1 << (b - b'0');
            if let Some(i) = m.get(&t) {
                result = result.max(j - i);
            } else {
                m.insert(t, j);
            }
            for k in 0..10 {
                let t1 = t ^ (1 << k);
                if let Some(i) = m.get(&t1) {
                    result = result.max(j - i);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::longest_awesome("3242415".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::longest_awesome("12345678".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(6, Solution::longest_awesome("213123".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(2, Solution::longest_awesome("00".to_string()));
    }
}
