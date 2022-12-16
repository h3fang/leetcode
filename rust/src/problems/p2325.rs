pub struct Solution;

impl Solution {
    pub fn decode_message(key: String, mut message: String) -> String {
        let mut map = [-1; 26];
        let mut i = 0;
        for &b in key.as_bytes() {
            if b == b' ' {
                continue;
            }
            let j = (b - b'a') as usize;
            if map[j] == -1 {
                map[j] = i;
                i += 1;
            }
        }

        let m = unsafe { message.as_bytes_mut() };
        for e in m {
            if *e == b' ' {
                continue;
            }
            let i = (*e - b'a') as usize;
            *e = map[i] as u8 + b'a';
        }
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let key = "the quick brown fox jumps over the lazy dog".into();
        let message = "vkbs bs t suepuv".into();
        assert_eq!("this is a secret", Solution::decode_message(key, message));
    }
}
