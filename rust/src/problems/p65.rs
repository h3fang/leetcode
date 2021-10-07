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
        assert_eq!(Solution::is_number("0".into()), true);
    }

    #[test]
    fn case2() {
        assert_eq!(Solution::is_number("e".into()), false);
    }

    #[test]
    fn case3() {
        assert_eq!(Solution::is_number("inf".into()), false);
    }

    #[test]
    fn case4() {
        assert_eq!(Solution::is_number("-8115e957".into()), true);
    }

    #[test]
    fn case5() {
        assert_eq!(Solution::is_number("0e".into()), false);
    }
}
