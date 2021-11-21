pub struct Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> char {
        let mut map = [(0usize, 0usize); 26];
        for (i, c) in s.as_bytes().iter().enumerate() {
            let m = &mut map[(c - b'a') as usize];
            if m.0 == 0 {
                m.1 = i;
            }
            m.0 += 1;
        }
        let mut result = None;
        let mut min_pos = usize::MAX;
        for (i, e) in map.iter().enumerate() {
            if e.0 == 1 && e.1 < min_pos {
                min_pos = e.1;
                result = Some((i as u8 + b'a') as char);
            }
        }
        result.unwrap_or(' ')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!('b', Solution::first_uniq_char("abaccdeff".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(' ', Solution::first_uniq_char("".to_string()));
    }
}
