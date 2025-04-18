pub struct Solution;

impl Solution {
    pub fn maximum_binary_string(mut binary: String) -> String {
        if let Some(i) = binary.as_bytes().iter().position(|&b| b == b'0') {
            let zeros = binary.as_bytes().iter().filter(|&&b| b == b'0').count();
            let b = unsafe { binary.as_bytes_mut() };
            b.iter_mut().for_each(|b| *b = b'1');
            b[i + zeros - 1] = b'0';
        }
        binary
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "111011",
            Solution::maximum_binary_string("000110".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!("01", Solution::maximum_binary_string("01".to_string()));
    }
}
