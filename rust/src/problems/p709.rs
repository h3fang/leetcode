pub struct Solution;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "Hello".to_string();
        assert_eq!("hello", Solution::to_lower_case(s));
    }
}
