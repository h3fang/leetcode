pub struct Solution;

impl Solution {
    pub fn count_vowel_substrings(word: String) -> i32 {
        fn is_vowel(b: u8) -> bool {
            b == b'a' || b == b'e' || b == b'i' || b == b'o' || b == b'u'
        }
        let w = word.as_bytes();
        let n = w.len();
        if n < 5 {
            return 0;
        }
        let mut result = 0;
        let mut j = 0;
        let mut k = 0;
        let mut vowels = 0;
        let mut count = [0i32; 256];

        for i in 0..n {
            if is_vowel(w[i]) {
                count[w[i] as usize] += 1;
                if count[w[i] as usize] == 1 {
                    vowels += 1;
                }
                while vowels == 5 {
                    count[w[k] as usize] -= 1;
                    if count[w[k] as usize] == 0 {
                        vowels -= 1;
                    }
                    k += 1;
                }
                result += k - j;
            } else {
                count[b'a' as usize] = 0;
                count[b'e' as usize] = 0;
                count[b'i' as usize] = 0;
                count[b'o' as usize] = 0;
                count[b'u' as usize] = 0;
                vowels = 0;
                j = i + 1;
                k = i + 1;
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::count_vowel_substrings("aeiouu".into()))
    }

    #[test]
    fn case2() {
        assert_eq!(0, Solution::count_vowel_substrings("unicornarihan".into()))
    }

    #[test]
    fn case3() {
        assert_eq!(7, Solution::count_vowel_substrings("cuaieuouac".into()))
    }
}
