pub struct Solution;

impl Solution {
    pub fn last_substring(s: String) -> String {
        let t = s.as_bytes();
        let n = t.len();
        let (mut i, mut j) = (0, 1);
        while j < n {
            let mut k = 0;
            while j + k < n && t[i + k] == t[j + k] {
                k += 1;
            }
            if j + k < n && t[i + k] < t[j + k] {
                i += k + 1;
                j = j.max(i + 1);
            } else {
                j += k + 1;
            }
        }
        s[i..].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("bab", Solution::last_substring("abab".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("tcode", Solution::last_substring("leetcode".to_string()));
    }
}
