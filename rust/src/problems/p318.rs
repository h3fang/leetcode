use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut masks: HashMap<u32, usize> = HashMap::new();
        let mut result = 0;
        for w in &words {
            let mut mask = 0u32;
            let n = w.len();
            for b in w.as_bytes() {
                mask |= 1 << (b - b'a');
            }
            let c = masks
                .iter()
                .map(|next| if mask & next.0 > 0 { 0 } else { n * next.1 })
                .max()
                .unwrap_or(0);
            result = result.max(c);
            let e = masks.entry(mask).or_default();
            *e = n.max(*e);
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abcw", "baz", "foo", "bar", "xtfn", "abcdef"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(16, Solution::max_product(words));
    }

    #[test]
    fn case2() {
        let words = ["a", "ab", "abc", "d", "cd", "bcd", "abcd"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(4, Solution::max_product(words));
    }

    #[test]
    fn case3() {
        let words = ["a", "aa", "aaa", "aaaa"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!(0, Solution::max_product(words));
    }
}
