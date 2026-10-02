pub struct Solution;

impl Solution {
    pub fn maximum_odd_binary_number(mut s: String) -> String {
        let n = s.len();
        let ones = s.bytes().filter(|&b| b == b'1').count();
        let bytes = unsafe { s.as_bytes_mut() };
        bytes[0..ones - 1].iter_mut().for_each(|b| *b = b'1');
        bytes[ones - 1..n - 1].iter_mut().for_each(|b| *b = b'0');
        bytes[n - 1] = b'1';
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "001",
            Solution::maximum_odd_binary_number("010".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "1001",
            Solution::maximum_odd_binary_number("0101".to_string())
        );
    }
}
