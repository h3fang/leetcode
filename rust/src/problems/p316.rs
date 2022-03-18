pub struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut last_index = [usize::MAX; 26];
        for (i, c) in s.char_indices() {
            let idx = (c as u8 - b'a') as usize;
            last_index[idx] = i;
        }
        let mut visited = [false; 26];
        let mut stack = vec![];
        for (i, c) in s.char_indices() {
            let idx = (c as u8 - b'a') as usize;
            if visited[idx] {
                continue;
            }
            while let Some(&top) = stack.last() {
                let idx_top = (top as u8 - b'a') as usize;
                if top > c && last_index[idx_top] > i {
                    stack.pop();
                    visited[idx_top] = false;
                } else {
                    break;
                }
            }
            stack.push(c);
            visited[idx] = true;
        }
        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "abc",
            Solution::remove_duplicate_letters("bcabc".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "acdb",
            Solution::remove_duplicate_letters("cbacdcbc".to_string())
        );
    }
}
