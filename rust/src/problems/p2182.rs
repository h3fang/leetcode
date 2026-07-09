pub struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut freq = [0; 26];
        for &b in s.as_bytes() {
            freq[(b - b'a') as usize] += 1;
        }
        let mut result = String::with_capacity(s.len());
        let (mut i, mut j, mut k) = (25, 24, 0);
        while i >= 0 && j >= 0 {
            if freq[i as usize] == 0 {
                i -= 1;
                k = 0;
            } else if k < repeat_limit {
                freq[i as usize] -= 1;
                k += 1;
                result.push((b'a' + i as u8) as char);
            } else if j >= i || freq[j as usize] == 0 {
                j -= 1;
            } else {
                freq[j as usize] -= 1;
                k = 0;
                result.push((b'a' + j as u8) as char);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "zzcccac",
            Solution::repeat_limited_string("cczazcc".to_string(), 3)
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "bbabaa",
            Solution::repeat_limited_string("aababab".to_string(), 2)
        );
    }
}
