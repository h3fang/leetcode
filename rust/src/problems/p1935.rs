pub struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut broken = 0u32;
        for b in broken_letters.as_bytes() {
            broken |= 1 << (b - b'a');
        }
        let mut ans = 0;
        for w in text.split_ascii_whitespace() {
            if w.as_bytes().iter().all(|b| (1 << (b - b'a')) & broken == 0) {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::can_be_typed_words("hello world".to_string(), "ad".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            1,
            Solution::can_be_typed_words("leet code".to_string(), "lt".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::can_be_typed_words("leet code".to_string(), "e".to_string())
        );
    }
}
