pub struct Solution;

impl Solution {
    pub fn replace_space(s: String) -> String {
        let mut r = String::new();
        for &b in s.as_bytes() {
            if b == b' ' {
                r.push_str("%20");
            } else {
                r.push(b as char);
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "We are happy.".to_string();
        let expected = "We%20are%20happy.".to_string();
        assert_eq!(expected, Solution::replace_space(s));
    }
}
