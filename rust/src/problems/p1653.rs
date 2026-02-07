pub struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let (mut f, mut b) = (0, 0);
        for &x in s.as_bytes() {
            let x = (x - b'a') as i32;
            b += x;
            f = (f + (1 ^ x)).min(b);
        }
        f
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
