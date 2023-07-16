pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut n1 = num1.into_bytes();
        let mut n2 = num2.into_bytes();
        let mut n = Vec::with_capacity(n1.len().max(n2.len()) + 1);
        let mut c = 0;
        while !n1.is_empty() || !n2.is_empty() || c != 0 {
            let d = c + n1.pop().unwrap_or(b'0') - b'0' + n2.pop().unwrap_or(b'0') - b'0';
            n.push(d % 10 + b'0');
            c = d / 10;
        }
        n.reverse();
        unsafe { String::from_utf8_unchecked(n) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "134",
            Solution::add_strings("11".to_string(), "123".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "533",
            Solution::add_strings("456".to_string(), "77".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!("0", Solution::add_strings("0".to_string(), "0".to_string()));
    }
}
