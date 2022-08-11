pub struct Solution;

impl Solution {
    pub fn reformat(s: String) -> String {
        let (mut digits, mut lower): (Vec<_>, Vec<_>) = s.chars().partition(|c| c.is_ascii_digit());
        if lower.len() < digits.len() {
            std::mem::swap(&mut digits, &mut lower);
        }
        if lower.len() - digits.len() > 1 {
            "".to_string()
        } else {
            let mut result = String::with_capacity(digits.len() + lower.len());
            for (&a, &b) in lower.iter().zip(&digits) {
                result.push(a);
                result.push(b);
            }
            if digits.len() < lower.len() {
                result.push(*lower.last().unwrap() as char);
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_valid(s: &str) {
        assert!(s.as_bytes().windows(2).all(|w| {
            (w[0].is_ascii_digit() && w[1].is_ascii_lowercase())
                || (w[1].is_ascii_digit() && w[0].is_ascii_lowercase())
        }))
    }

    #[test]
    fn case1() {
        assert_valid(&Solution::reformat("a0b1c2".to_string()));
    }
}
