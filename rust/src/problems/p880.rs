pub struct Solution;

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let mut n = 0;
        for &b in s {
            if b.is_ascii_digit() {
                n *= (b - b'0') as usize;
            } else {
                n += 1;
            }
        }
        let mut k = k as usize;
        for &b in s.iter().rev() {
            k %= n;
            if b.is_ascii_alphabetic() && k == 0 {
                return (b as char).to_string();
            }
            if b.is_ascii_digit() {
                n /= (b - b'0') as usize;
            } else {
                n -= 1;
            }
        }
        unreachable!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("o", Solution::decode_at_index("leet2code3".to_string(), 10));
    }

    #[test]
    fn case2() {
        assert_eq!("h", Solution::decode_at_index("ha22".to_string(), 5));
    }

    #[test]
    fn case3() {
        assert_eq!(
            "a",
            Solution::decode_at_index("a2345678999999999999999".to_string(), 1)
        );
    }
}
