pub struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        fn is_vowel(b: u8) -> bool {
            matches!(b, b'a' | b'e' | b'i' | b'o' | b'u')
        }
        let k = k as usize;
        let s = s.as_bytes();
        let mut vowels = s[..k].iter().filter(|&&b| is_vowel(b)).count() as i32;
        let mut result = vowels;
        for (i, &b) in s.iter().enumerate().skip(k) {
            if is_vowel(b) {
                vowels += 1;
            }
            if is_vowel(s[i - k]) {
                vowels -= 1;
            }
            result = result.max(vowels);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_vowels("abciiidef".to_string(), 3));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::max_vowels("aeiou".to_string(), 2));
    }

    #[test]
    fn case3() {
        assert_eq!(2, Solution::max_vowels("leetcode".to_string(), 3));
    }
}
