pub struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        num.bytes().enumerate().fold(0, |acc, (i, b)| {
            let b = (b - b'0') as i32;
            acc + if i % 2 == 0 { b } else { -b }
        }) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(!Solution::is_balanced("1234".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::is_balanced("24123".to_string()));
    }
}
