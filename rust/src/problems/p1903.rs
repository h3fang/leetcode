pub struct Solution;

impl Solution {
    pub fn largest_odd_number(num: String) -> String {
        (0..num.len())
            .rev()
            .find(|&i| (num.as_bytes()[i] - b'0') % 2 == 1)
            .map(|i| num[..=i].to_string())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("5", Solution::largest_odd_number("52".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::largest_odd_number("4206".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!("35427", Solution::largest_odd_number("35427".to_string()));
    }
}
