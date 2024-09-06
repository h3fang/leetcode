pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.to_lowercase();
        let parts = s.split('e').collect::<Vec<_>>();
        let is_decimal = |s: &str| {
            if let Ok(n) = s.parse::<f64>() {
                n.is_finite()
            } else {
                false
            }
        };

        match parts.len() {
            1 => is_decimal(parts[0]),
            2 => is_decimal(parts[0]) && parts[1].parse::<i64>().is_ok(),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::is_number("0".into()));
    }

    #[test]
    fn case2() {
        assert!(!Solution::is_number("e".into()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::is_number("inf".into()));
    }

    #[test]
    fn case4() {
        assert!(Solution::is_number("-8115e957".into()));
    }

    #[test]
    fn case5() {
        assert!(!Solution::is_number("0e".into()));
    }
}
