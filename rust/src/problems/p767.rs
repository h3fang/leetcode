pub struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n < 2 {
            return s;
        }
        let mut cnt = [0; 26];
        let mut max = 0;
        for &b in bytes {
            let i = (b - b'a') as usize;
            cnt[i] += 1;
            max = max.max(cnt[i]);
        }
        if max > (n + 1) / 2 {
            return String::new();
        }
        let mut result = vec![0; n];
        let (mut even, mut odd, half) = (0, 1, n / 2);
        for (i, e) in cnt.iter_mut().enumerate() {
            let c = b'a' + i as u8;
            while *e > 0 && *e <= half && odd < n {
                result[odd] = c;
                *e -= 1;
                odd += 2;
            }
            while *e > 0 {
                result[even] = c;
                *e -= 1;
                even += 2;
            }
        }
        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let result = Solution::reorganize_string("aab".to_string());
        assert!(result.as_bytes().windows(2).all(|w| w[0] != w[1]));
    }

    #[test]
    fn case2() {
        let result = Solution::reorganize_string("aaab".to_string());
        assert!(result.is_empty());
    }
}
