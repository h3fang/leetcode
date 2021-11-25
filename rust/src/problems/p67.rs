pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.into_bytes();
        let mut b = b.into_bytes();

        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }
        let mut carry = b'0';
        for (a, b) in a.iter_mut().rev().zip(b.iter().rev()) {
            match (*a, *b) {
                (b'0', b'0') => {
                    *a = carry;
                    carry = b'0';
                }
                (b'1', b'1') => {
                    *a = carry;
                    carry = b'1';
                }
                _ => {
                    if carry == b'0' {
                        *a = b'1';
                    } else {
                        *a = b'0';
                    }
                }
            }
        }

        for i in (0..(a.len() - b.len())).rev() {
            match (a[i], carry) {
                (b'0', _) => {
                    a[i] = carry;
                    carry = b'0';
                    break;
                }
                (b'1', b'1') => {
                    a[i] = b'0';
                    carry = b'1';
                }
                _ => break,
            }
        }
        if carry == b'1' {
            a.insert(0, b'1');
        }
        unsafe { String::from_utf8_unchecked(a) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!("100".to_string(), Solution::add_binary(a, b));
    }

    #[test]
    fn case2() {
        let a = "1010".to_string();
        let b = "1011".to_string();
        assert_eq!("10101".to_string(), Solution::add_binary(a, b));
    }
}
