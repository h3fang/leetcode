pub struct Solution;

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        let mut m = [[false; 2]; 26];
        let mut result = -1;
        for c in s.as_bytes() {
            let i = (c.to_ascii_lowercase() - b'a') as usize;
            if c.is_ascii_lowercase() {
                m[i][0] = true;
            } else {
                m[i][1] = true;
            }
            if m[i][0] && m[i][1] && i as i32 > result {
                result = i as i32;
            }
        }
        if result == -1 {
            "".into()
        } else {
            let c = (result as u8 + b'A') as char;
            c.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("E", Solution::greatest_letter("lEeTcOdE".into()))
    }
}
