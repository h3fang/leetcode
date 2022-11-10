pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let mut r = String::with_capacity(s.len());
        for &b in s.as_bytes() {
            if !r.is_empty() && *r.as_bytes().last().unwrap() == b {
                r.pop();
            } else {
                r.push(b as char);
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("ca", Solution::remove_duplicates("abbaca".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("ay", Solution::remove_duplicates("azxxzy".to_string()));
    }
}
