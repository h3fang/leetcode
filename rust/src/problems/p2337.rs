pub struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let s = start.as_bytes();
        let t = target.as_bytes();
        if s.len() != t.len() {
            return false;
        }
        let n = s.len();

        let mut i = 0;
        let mut j = 0;
        while i < n && j < n {
            while i < n && s[i] == b'_' {
                i += 1;
            }
            while j < n && t[j] == b'_' {
                j += 1;
            }
            if i == n || j == n {
                break;
            }
            if s[i] != t[j] || ((s[i] == b'L' && i < j) || (s[i] == b'R' && i > j)) {
                return false;
            }
            i += 1;
            j += 1;
        }
        s[i..].iter().chain(&t[j..]).all(|&c| c == b'_')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::can_change("_L__R__R_".into(), "L______RR".into()));
    }
}
