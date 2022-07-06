use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        fn skip_spaces(exp: &[u8], i: &mut usize) {
            while exp[*i] == b' ' {
                *i += 1;
            }
        }

        fn parse_var<'a>(exp: &'a [u8], i: &mut usize) -> &'a [u8] {
            let n = exp.len();
            let a = *i;
            while *i < n && exp[*i] != b' ' && exp[*i] != b')' {
                *i += 1;
            }
            &exp[a..*i]
        }

        fn parse_int(exp: &[u8], i: &mut usize) -> i32 {
            let n = exp.len();
            let mut sign = 1;
            let mut result = 0;
            if exp[*i] == b'-' {
                sign = -1;
                *i += 1;
            }
            while *i < n && exp[*i].is_ascii_digit() {
                result = result * 10 + (exp[*i] - b'0') as i32;
                *i += 1;
            }
            sign * result
        }

        fn eval(exp: &[u8], i: &mut usize, stack: &mut HashMap<Vec<u8>, Vec<i32>>) -> i32 {
            skip_spaces(exp, i);
            if exp[*i] != b'(' {
                if exp[*i].is_ascii_alphabetic() {
                    let var = parse_var(exp, i);
                    return *stack.get(var).unwrap().last().unwrap();
                } else {
                    return parse_int(exp, i);
                }
            }
            *i += 1;
            skip_spaces(exp, i);
            let result;
            match exp[*i] {
                b'l' => {
                    *i += 3;
                    skip_spaces(exp, i);
                    let mut vars = vec![];
                    loop {
                        if !exp[*i].is_ascii_alphabetic() {
                            result = eval(exp, i, stack);
                            break;
                        }
                        let v = parse_var(exp, i);
                        skip_spaces(exp, i);
                        if exp[*i] == b')' {
                            result = *stack.get(v).unwrap().last().unwrap();
                            break;
                        }
                        vars.push(v);
                        let b = eval(exp, i, stack);
                        stack.entry(v.to_vec()).or_default().push(b);
                        skip_spaces(exp, i);
                    }

                    for var in vars {
                        stack.get_mut(var).unwrap().pop();
                    }
                }
                b'a' => {
                    *i += 3;
                    skip_spaces(exp, i);
                    let a = eval(exp, i, stack);
                    skip_spaces(exp, i);
                    let b = eval(exp, i, stack);
                    skip_spaces(exp, i);
                    result = a + b;
                }
                b'm' => {
                    *i += 4;
                    skip_spaces(exp, i);
                    let a = eval(exp, i, stack);
                    skip_spaces(exp, i);
                    let b = eval(exp, i, stack);
                    skip_spaces(exp, i);
                    result = a * b;
                }
                _ => unreachable!(),
            }
            *i += 1;
            result
        }

        eval(expression.as_bytes(), &mut 0, &mut HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            14,
            Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".into())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::evaluate("(let x 3 x 2 x)".into()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            5,
            Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".into())
        );
    }
}
