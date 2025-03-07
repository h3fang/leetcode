pub struct Solution;

impl Solution {
    pub fn smallest_string(mut s: String) -> String {
        let t = unsafe { s.as_bytes_mut() };
        for i in 0..t.len() {
            if t[i] > b'a' {
                t[i] -= 1;
                for b in &mut t[i + 1..] {
                    if *b == b'a' {
                        break;
                    }
                    *b -= 1;
                }
                break;
            } else if i == t.len() - 1 {
                t[i] = b'z';
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!("baabc", Solution::smallest_string("cbabc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!("abaab", Solution::smallest_string("acbbc".to_string()));
    }

    #[test]
    fn case3() {
        assert_eq!(
            "kddsbncd",
            Solution::smallest_string("leetcode".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!("z", Solution::smallest_string("a".to_string()));
    }
}
