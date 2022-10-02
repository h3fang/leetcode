pub struct Solution;

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        let s = start.as_bytes();
        let e = end.as_bytes();
        let n = s.len();
        let mut i = 0;
        let mut j = 0;
        while i < n && j < n {
            while i < n && s[i] == b'X' {
                i += 1;
            }
            while j < n && e[j] == b'X' {
                j += 1;
            }
            if i == n || j == n {
                break;
            }
            if s[i] != e[j] {
                return false;
            }
            if (s[i] == b'L' && i < j) || (s[i] == b'R' && i > j) {
                return false;
            }
            i += 1;
            j += 1;
        }
        if s[i..].iter().any(|&e| e != b'X') || e[j..].iter().any(|&e| e != b'X') {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::can_transform("RXXLRXRXL".to_string(), "XRLXXRRLX".to_string())
        );
    }
}
