pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn equal_frequency(word: String) -> bool {
        let mut f = [0i32; 26];
        for b in word.as_bytes() {
            let i = (b - b'a') as usize;
            f[i] += 1;
        }
        let mut m = HashMap::new();
        for e in f {
            if e > 0 {
                *m.entry(e).or_insert(0) += 1;
            }
        }
        if f.iter().all(|&e| e == 0 || e == 1) || f.iter().filter(|e| **e > 0).count() == 1 {
            return true;
        }
        if m.len() != 2 {
            return false;
        }
        let s = m.into_iter().collect::<Vec<_>>();
        if (s[0].0 - s[1].0 == 1 && s[0].1 == 1)
            || (s[1].0 - s[0].0 == 1 && s[1].1 == 1)
            || (s[0].0 == 1 && s[0].1 == 1)
            || (s[1].0 == 1 && s[1].1 == 1)
        {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(true, Solution::equal_frequency("abcc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(false, Solution::equal_frequency("aazz".to_string()));
    }
}
