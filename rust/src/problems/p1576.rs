pub struct Solution;

impl Solution {
    pub fn modify_string(mut s: String) -> String {
        let n = s.len();
        let bytes = unsafe { s.as_bytes_mut() };
        for i in 0..n {
            if bytes[i] == b'?' {
                for b in b'a'..=b'c' {
                    if (i == 0 || bytes[i - 1] != b) && (i + 1 == n || bytes[i + 1] != b) {
                        bytes[i] = b;
                        break;
                    }
                }
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(s: &str) -> bool {
        println!("{}", s);
        !s.contains('?') && s.as_bytes().windows(2).all(|w| w[0] != w[1])
    }

    #[test]
    fn case1() {
        assert!(is_valid(&Solution::modify_string("?zs".to_string())));
    }

    #[test]
    fn case2() {
        assert!(is_valid(&Solution::modify_string("j?qg??b".to_string())));
    }

    #[test]
    fn case3() {
        assert!(is_valid(&Solution::modify_string("???".to_string())));
    }
}
