pub struct Solution;

impl Solution {
    pub fn break_palindrome(mut palindrome: String) -> String {
        let p = unsafe { palindrome.as_bytes_mut() };
        if p[..p.len() / 2].iter().all(|e| *e == b'a') {
            if p.len() == 1 {
                return "".to_string();
            } else {
                p[p.len() - 1] = b'b';
            }
        } else {
            for i in 0..=p.len() / 2 {
                if p[i] > b'a' {
                    p[i] = b'a';
                    break;
                }
            }
        }
        palindrome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("aaccba", Solution::break_palindrome("abccba".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("", Solution::break_palindrome("a".to_string()));
    }
}
