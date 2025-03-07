pub struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let t = t.as_bytes();
        let mut j = 0;
        let n = t.len();
        for b in s.bytes() {
            if b == t[j] {
                j += 1;
                if j == n {
                    break;
                }
            }
        }
        (n - j) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            4,
            Solution::append_characters("coaching".to_string(), "coding".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::append_characters("abcde".to_string(), "a".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            5,
            Solution::append_characters("z".to_string(), "abcde".to_string())
        );
    }
}
