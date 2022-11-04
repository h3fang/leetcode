pub struct Solution;

const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let a = unsafe { s.as_bytes_mut() };
        let n = a.len();
        let mut i = 0;
        let mut j = a.len() - 1;
        'outter: while i < j {
            while i < n && !VOWELS.contains(&a[i].to_ascii_lowercase()) {
                i += 1;
                if i >= j {
                    break 'outter;
                }
            }

            while !VOWELS.contains(&a[j].to_ascii_lowercase()) {
                if j > 0 {
                    j -= 1;
                } else {
                    break 'outter;
                }
                if i >= j {
                    break 'outter;
                }
            }
            a.swap(i, j);
            i += 1;
            j -= 1;
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("holle", Solution::reverse_vowels("hello".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("leotcede", Solution::reverse_vowels("leetcode".to_string()));
    }
}
