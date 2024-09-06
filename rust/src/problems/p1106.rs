pub struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        fn parse_and(mut exp: &[u8]) -> (bool, &[u8]) {
            let mut result = true;
            loop {
                let (r, e) = parse(exp);
                result &= r;
                let next = e[0];
                exp = &e[1..];
                if next == b')' {
                    break;
                }
            }
            (result, exp)
        }

        fn parse_or(mut exp: &[u8]) -> (bool, &[u8]) {
            let mut result = false;
            loop {
                let (r, e) = parse(exp);
                result |= r;
                let next = e[0];
                exp = &e[1..];
                if next == b')' {
                    break;
                }
            }
            (result, exp)
        }

        fn parse_not(exp: &[u8]) -> (bool, &[u8]) {
            let (r, e) = parse(exp);
            (!r, &e[1..])
        }

        fn parse(exp: &[u8]) -> (bool, &[u8]) {
            match exp[0] {
                b'!' => parse_not(&exp[2..]),
                b'&' => parse_and(&exp[2..]),
                b'|' => parse_or(&exp[2..]),
                b'f' => (false, &exp[1..]),
                b't' => (true, &exp[1..]),
                b',' => parse(&exp[1..]),
                _ => unreachable!("{}", unsafe { std::str::from_utf8_unchecked(exp) }),
            }
        }
        parse(expression.as_bytes()).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::parse_bool_expr("!(f)".to_string()));
    }

    #[test]
    fn case2() {
        assert!(Solution::parse_bool_expr("|(f,t)".to_string()));
    }

    #[test]
    fn case3() {
        assert!(!Solution::parse_bool_expr("&(t,f)".to_string()));
    }

    #[test]
    fn case4() {
        assert!(!Solution::parse_bool_expr("|(&(t,f,t),!(t))".to_string()));
    }

    #[test]
    fn case5() {
        assert!(Solution::parse_bool_expr(
            "!(&(&(!(&(f)),&(t),|(f,f,t)),&(t),&(t,t,f)))".to_string()
        ));
    }
}
