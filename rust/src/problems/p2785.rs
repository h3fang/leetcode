pub struct Solution;

impl Solution {
    pub fn sort_vowels(mut s: String) -> String {
        let n = s.len();
        let (mut pos, mut vowels) = (Vec::with_capacity(n), Vec::with_capacity(n));
        for (i, &b) in s.as_bytes().iter().enumerate() {
            if b"aeiouAEIOU".contains(&b) {
                pos.push(i);
                vowels.push(b);
            }
        }
        vowels.sort_unstable();
        let t = unsafe { s.as_bytes_mut() };
        for (i, b) in pos.into_iter().zip(vowels) {
            t[i] = b;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("lEOtcede", Solution::sort_vowels("lEetcOde".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("lYmpH", Solution::sort_vowels("lYmpH".to_string()));
    }
}
