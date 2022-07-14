pub struct Solution;

impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        let s = start.replace('_', "");
        let t = target.replace('_', "");
        if s != t {
            return false;
        }
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
            if i == n && j == n {
                break;
            }
            if i != j && ((s[i] == b'L' && i < j) || (s[i] == b'R' && i > j)) {
                return false;
            }
            i += 1;
            j += 1;
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
            Solution::can_change("_L__R__R_".into(), "L______RR".into())
        );
    }
}
