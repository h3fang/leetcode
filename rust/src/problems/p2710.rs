pub struct Solution;

impl Solution {
    pub fn remove_trailing_zeros(mut num: String) -> String {
        let s = unsafe { num.as_mut_vec() };
        while !s.is_empty() && *s.last().unwrap() == b'0' {
            s.pop();
        }
        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "512301".to_string(),
            Solution::remove_trailing_zeros("51230100".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "123".to_string(),
            Solution::remove_trailing_zeros("123".to_string())
        );
    }
}
