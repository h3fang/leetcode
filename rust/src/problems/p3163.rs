pub struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut result = Vec::with_capacity(word.len());
        let (mut prev, mut c) = (word.as_bytes()[0], 1u8);
        for b in word.bytes().skip(1).chain([b'!']) {
            if b == prev {
                if c == 9 {
                    result.extend([b'0' + c, b]);
                    c = 1;
                } else {
                    c += 1;
                }
            } else {
                result.extend([b'0' + c, prev]);
                prev = b;
                c = 1;
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
        assert_eq!(
            "1a1b1c1d1e",
            Solution::compressed_string("abcde".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "9a5a2b",
            Solution::compressed_string("aaaaaaaaaaaaaabb".to_string())
        );
    }
}
