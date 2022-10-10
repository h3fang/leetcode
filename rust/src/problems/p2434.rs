pub struct Solution;

impl Solution {
    pub fn robot_with_string(s: String) -> String {
        let mut f = vec![(b'z' + 1) as char; s.len() + 1];
        for (i, c) in s.char_indices().rev() {
            f[i] = f[i + 1].min(c);
        }
        let mut r = Vec::with_capacity(s.len());
        let mut result = String::with_capacity(s.len());
        for (i, c) in s.char_indices() {
            r.push(c);
            while !r.is_empty() && *r.last().unwrap() <= f[i + 1] {
                result.push(r.pop().unwrap());
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("azz", Solution::robot_with_string("zza".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("abc", Solution::robot_with_string("bac".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!("addb", Solution::robot_with_string("bdda".to_string()));
    }
}
