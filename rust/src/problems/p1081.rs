pub struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let mut last_index = [usize::MAX; 26];
        for (i, c) in s.char_indices() {
            let idx = (c as u8 - b'a') as usize;
            last_index[idx] = i;
        }
        let mut visited = 0u32;
        let mut stack = vec![];
        for (i, c) in s.char_indices() {
            let idx = c as u8 - b'a';
            if visited & (1 << idx) > 0 {
                continue;
            }
            while let Some(&top) = stack.last() {
                let idx_top = (top as u8 - b'a') as usize;
                if top > c && last_index[idx_top] > i {
                    stack.pop();
                    visited &= !(1 << idx_top);
                } else {
                    break;
                }
            }
            stack.push(c);
            visited |= 1 << idx;
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("abc", Solution::smallest_subsequence("bcabc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            "acdb",
            Solution::smallest_subsequence("cbacdcbc".to_string())
        );
    }
}
