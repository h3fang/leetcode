pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut m: HashMap<u8, Vec<i32>> = HashMap::new();
        for (i, &c) in s.iter().enumerate() {
            m.entry(c).or_default().push(i as i32);
        }
        let mut result = 0;
        for v in m.values_mut() {
            v.insert(0, -1);
            v.push(n as i32);
            for i in 1..v.len() - 1 {
                result += (v[i] - v[i - 1]) * (v[i + 1] - v[i]);
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
        assert_eq!(10, Solution::unique_letter_string("ABC".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(8, Solution::unique_letter_string("ABA".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(92, Solution::unique_letter_string("LEETCODE".to_string()));
    }
}
