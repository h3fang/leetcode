pub struct Solution;

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let (left, right) = equation.split_once('=').unwrap();

        fn parse_expression(exp: &str) -> (i32, i32) {
            let mut a = 0;
            let mut b = 0;
            let mut sign = 1;
            let mut num = 0;
            for (i, &c) in exp.as_bytes().iter().chain(b"+").enumerate() {
                match c {
                    b'+' => {
                        b += sign * num;
                        num = 0;
                        sign = 1;
                    }
                    b'-' => {
                        b += sign * num;
                        num = 0;
                        sign = -1;
                    }
                    b'x' => {
                        if i > 0 && exp.as_bytes()[i - 1].is_ascii_digit() {
                            a += sign * num;
                        } else {
                            a += sign;
                        }
                        num = 0;
                    }
                    n => {
                        num = num * 10 + (n - b'0') as i32;
                    }
                }
            }
            (a, b)
        }

        let (a1, b1) = parse_expression(left);
        let (a2, b2) = parse_expression(right);
        let a = a1 - a2;
        let b = b1 - b2;
        if a == 0 && b == 0 {
            "Infinite solutions".to_string()
        } else if a == 0 {
            "No solution".to_string()
        } else {
            format!("x={}", -b / a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("x=2", Solution::solve_equation("x+5-3+x=6+x-2".into()));
    }

    #[test]
    fn case2() {
        assert_eq!("Infinite solutions", Solution::solve_equation("x=x".into()));
    }
}
