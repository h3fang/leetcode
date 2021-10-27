use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        fn bt(
            s: &[u8],
            left: usize,
            right: usize,
            l_rem: usize,
            r_rem: usize,
            curr: &mut Vec<u8>,
            result: &mut HashSet<String>,
        ) {
            if s.is_empty() {
                if l_rem == 0 && r_rem == 0 {
                    unsafe {
                        result.insert(String::from_utf8_unchecked(curr.clone()));
                    }
                    result.insert(String::from_utf8_lossy(curr).to_string());
                }
            } else {
                match s[0] {
                    b'(' => {
                        if s.len() > left + 1 - right + l_rem + r_rem {
                            curr.push(b'(');
                            bt(&s[1..], left + 1, right, l_rem, r_rem, curr, result);
                            curr.pop();
                        }
                        if l_rem > 0 {
                            bt(&s[1..], left, right, l_rem - 1, r_rem, curr, result);
                        }
                    }
                    b')' => {
                        if left > right && s.len() > left - right - 1 + l_rem + r_rem {
                            curr.push(b')');
                            bt(&s[1..], left, right + 1, l_rem, r_rem, curr, result);
                            curr.pop();
                        }
                        if r_rem > 0 {
                            bt(&s[1..], left, right, l_rem, r_rem - 1, curr, result);
                        }
                    }
                    x => {
                        curr.push(x);
                        bt(&s[1..], left, right, l_rem, r_rem, curr, result);
                        curr.pop();
                    }
                }
            }
        }

        let mut l_rem = 0;
        let mut r_rem = 0;
        for c in s.as_bytes() {
            if *c == b'(' {
                l_rem += 1;
            } else if *c == b')' {
                if l_rem == 0 {
                    r_rem += 1;
                } else {
                    l_rem -= 1;
                }
            }
        }
        let mut result = HashSet::new();
        let mut curr = Vec::new();
        bt(s.as_bytes(), 0, 0, l_rem, r_rem, &mut curr, &mut result);
        result.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "()())()".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = ["(())()", "()()()"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let s = "(a)())()".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = ["(a())()", "(a)()()"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case3() {
        let s = ")(".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = [""];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case4() {
        let s = "))".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = [""];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case5() {
        let s = ")()(".to_string();
        let mut result = Solution::remove_invalid_parentheses(s);
        result.sort_unstable();
        let expected = ["()"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
