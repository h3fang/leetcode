pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.len() > s.len() {
            return "".to_string();
        }

        let mut sig = [0; 256];
        for &c in t.as_bytes() {
            sig[c as usize] += 1;
        }

        let required = sig.iter().filter(|e| **e > 0).count();

        let mut filtered = Vec::new();
        for (i, &c) in s.as_bytes().iter().enumerate() {
            if sig[c as usize] > 0 {
                filtered.push((i, c));
            }
        }

        let mut left = 0;
        let mut min = usize::MAX;
        let mut result = None;

        let mut window = [0; 256];
        let mut valid = 0;

        for (right, &(i, c)) in filtered.iter().enumerate() {
            window[c as usize] += 1;

            if sig[c as usize] > 0 && window[c as usize] == sig[c as usize] {
                valid += 1;
            }

            while valid == required {
                let len = i - filtered[left].0 + 1;
                if len < min {
                    min = len;
                    result = Some((left, right));
                }
                let c = filtered[left].1 as usize;
                window[c] -= 1;
                if sig[c as usize] > 0 && window[c as usize] < sig[c as usize] {
                    valid -= 1;
                }
                left += 1;
            }
        }

        if let Some((left, right)) = result {
            let left = filtered[left].0;
            let right = filtered[right].0;
            s[left..=right].to_string()
        } else {
            "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let expected = "BANC".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let expected = "a".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let expected = "".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case4() {
        let s = "ab".to_string();
        let t = "b".to_string();
        let expected = "b".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }

    #[test]
    fn case5() {
        let s = "a".to_string();
        let t = "b".to_string();
        let expected = "".to_string();
        assert_eq!(expected, Solution::min_window(s, t));
    }
}
