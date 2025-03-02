pub struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits = s.into_bytes();
        digits.iter_mut().for_each(|x| *x -= b'0');
        while digits.len() > 2 {
            let mut next = Vec::with_capacity(digits.len() - 1);
            for w in digits.windows(2) {
                next.push((w[0] + w[1]) % 10);
            }
            digits = next;
        }
        digits[0] == digits[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::has_same_digits("3902".to_string()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::has_same_digits("34789".to_string()));
    }
}
