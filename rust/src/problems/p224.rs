pub struct Solution;

enum Op {
    Symbol(u8),
    Num(i32),
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ops = vec![];
        let mut curr = None;
        for &c in [b'('].iter().chain(s).chain(&[b')']) {
            match c {
                b'+' | b'-' | b'(' => {
                    if let Some(curr) = curr.take() {
                        ops.push(Op::Num(curr));
                    }
                    ops.push(Op::Symbol(c));
                }
                b')' => {
                    if let Some(curr) = curr.take() {
                        ops.push(Op::Num(curr));
                    }
                    let mut value = 0;
                    while let Some(op) = ops.pop() {
                        match op {
                            Op::Num(n) => {
                                let prev = ops.last().unwrap();
                                match prev {
                                    Op::Symbol(b'+') => {
                                        value += n;
                                        ops.pop();
                                    }
                                    Op::Symbol(b'-') => {
                                        value -= n;
                                        ops.pop();
                                    }
                                    Op::Symbol(b'(') => value += n,
                                    _ => unreachable!(),
                                }
                            }
                            Op::Symbol(b'(') => break,
                            _ => unreachable!(),
                        }
                    }
                    ops.push(Op::Num(value));
                }
                c if c.is_ascii_digit() => {
                    let digit = (c - b'0') as i32;
                    curr = if let Some(curr) = curr {
                        Some(curr * 10 + digit)
                    } else {
                        Some(digit)
                    };
                }
                _ => {}
            }
        }
        if let Some(&Op::Num(r)) = ops.last() {
            r
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::calculate("1 + 1".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(3, Solution::calculate(" 2-1 + 2 ".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(23, Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()));
    }
}
