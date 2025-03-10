pub struct Solution;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let (n, m) = (str1.len(), str2.len());
        let s2 = str2.as_bytes();
        let mut result = vec![b'a'; n + m - 1];
        let mut visited = vec![false; n + m - 1];
        for (i, &x) in str1.as_bytes().iter().enumerate() {
            if x == b'T' {
                for (j, &b) in s2.iter().enumerate() {
                    result[i + j] = b;
                    visited[i + j] = true;
                }
            }
        }
        for (i, &x) in str1.as_bytes().iter().enumerate() {
            if x == b'T' && &result[i..i + m] != s2 {
                return String::new();
            }
        }
        for (i, &x) in str1.as_bytes().iter().enumerate() {
            if x == b'F' && &result[i..i + m] == s2 {
                let mut done = false;
                for j in (i..i + m).rev() {
                    if !visited[j] {
                        result[j] = b'b';
                        done = true;
                        break;
                    }
                }
                if !done {
                    return String::new();
                }
            }
        }
        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            "ababa",
            Solution::generate_string("TFTF".to_string(), "ab".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            "",
            Solution::generate_string("TFTF".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            "a",
            Solution::generate_string("F".to_string(), "d".to_string())
        );
    }
}
