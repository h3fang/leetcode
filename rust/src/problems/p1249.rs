pub struct Solution;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut left = 0;
        let mut result = String::new();
        for c in s.chars() {
            match c {
                '(' => {
                    left += 1;
                    result.push('(');
                }
                ')' => {
                    if left > 0 {
                        left -= 1;
                        result.push(')');
                    }
                }
                _ => result.push(c),
            }
        }
        let r = unsafe { result.as_mut_vec() };
        for i in (0..r.len()).rev() {
            if left == 0 {
                break;
            }
            if r[i] == b'(' {
                r.remove(i);
                left -= 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(s: &str) -> bool {
        println!("{}", s);
        let mut left = 0;
        for c in s.chars() {
            match c {
                '(' => left += 1,
                ')' => {
                    if left > 0 {
                        left -= 1;
                    } else {
                        return false;
                    }
                }
                _ => {}
            }
        }
        left == 0
    }

    #[test]
    fn case1() {
        let result = Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string());
        assert!(is_valid(&result));
    }

    #[test]
    fn case2() {
        let result = Solution::min_remove_to_make_valid("a)b(c)d".to_string());
        assert!(is_valid(&result));
    }

    #[test]
    fn case3() {
        let result = Solution::min_remove_to_make_valid("())()(((".to_string());
        assert!(is_valid(&result));
    }

    #[test]
    fn case4() {
        let result = Solution::min_remove_to_make_valid("))((".to_string());
        assert!(is_valid(&result));
    }
}
