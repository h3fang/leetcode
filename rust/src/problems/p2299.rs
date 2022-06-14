pub struct Solution;

impl Solution {
    pub fn strong_password_checker_ii(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }
        let mut lower = 0;
        let mut upper = 0;
        let mut digit = 0;
        let mut special = 0;
        let mut last = ' ';
        for c in password.chars() {
            if c == last {
                return false;
            }
            last = c;
            if c.is_ascii_lowercase() {
                lower += 1;
            } else if c.is_ascii_uppercase() {
                upper += 1;
            } else if c.is_ascii_digit() {
                digit += 1;
            } else {
                special += 1;
            }
        }
        lower >= 1 && upper >= 1 && digit >= 1 && special >= 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::strong_password_checker_ii("IloveLe3tcode!".into())
        );
    }
}
