pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (mut a, b) = if a.len() < b.len() {
            (b.into_bytes(), a.into_bytes())
        } else {
            (a.into_bytes(), b.into_bytes())
        };

        let (n, mut carry) = (b.len(), 0);
        for (i, x) in a.iter_mut().rev().enumerate() {
            let y = if i < b.len() { b[n - 1 - i] - b'0' } else { 0 };
            carry += *x - b'0' + y;
            *x = (carry & 1) + b'0';
            carry /= 2;
        }
        if carry == 1 {
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
