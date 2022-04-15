pub struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        fn parse(s: &[u8], i: &mut usize) -> NestedInteger {
            if s[*i] == b'[' {
                *i += 1;
                let mut list = vec![];
                while s[*i] != b']' {
                    list.push(parse(s, i));
                    if s[*i] == b',' {
                        *i += 1;
                    }
                }
                *i += 1;
                NestedInteger::List(list)
            } else {
                let mut n = 0;
                let sign = if s[*i] == b'-' {
                    *i += 1;
                    -1
                } else {
                    1
                };

                while *i < s.len() && s[*i].is_ascii_digit() {
                    n = n * 10 + (s[*i] - b'0') as i32;
                    *i += 1;
                }
                NestedInteger::Int(sign * n)
            }
        }

        parse(s.as_bytes(), &mut 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use NestedInteger::{Int, List};

    #[test]
    fn case1() {
        assert_eq!(Int(324), Solution::deserialize("324".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            List(vec![Int(123), List(vec![Int(456), List(vec![Int(789)])])]),
            Solution::deserialize("[123,[456,[789]]]".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            List(vec![
                Int(123),
                Int(456),
                List(vec![Int(-788), Int(799), Int(833),]),
                List(vec![List(vec![])]),
                Int(10),
                List(vec![])
            ]),
            Solution::deserialize("[123,456,[-788,799,833],[[]],10,[]]".to_string())
        );
    }
}
