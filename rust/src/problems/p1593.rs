pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        fn bt<'a>(s: &'a [u8], words: &mut HashSet<&'a [u8]>) -> i32 {
            let mut max = 0;
            for i in 1..=s.len() {
                if !words.contains(&s[..i]) {
                    words.insert(&s[..i]);
                    max = max.max(1 + bt(&s[i..], words));
                    words.remove(&s[..i]);
                }
            }
            max
        }
        bt(s.as_bytes(), &mut HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(5, Solution::max_unique_split("ababccc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_unique_split("aba".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(1, Solution::max_unique_split("aa".to_string()));
    }
}
