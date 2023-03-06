pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut leftb = 0;
        let mut righta = s.as_bytes().iter().filter(|&&b| b == b'a').count() as i32;
        let mut result = righta;
        for &b in s.as_bytes() {
            if b == b'b' {
                leftb += 1;
            } else if b == b'a' {
                righta -= 1;
            }
            result = result.min(leftb + righta);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::minimum_deletions("aababbab".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(2, Solution::minimum_deletions("bbaaaaabb".to_string()));
    }
}
