pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let m = haystack.len();
        let n = needle.len();

        if n == 0 {
            return 0;
        }

        if m < n {
            return -1;
        }

        let haystack = haystack.as_bytes();
        let needle = needle.as_bytes();
        let mut next = vec![0; n];
        let mut j = 0;
        for i in 1..n {
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }
            if needle[i] == needle[j] {
                j += 1;
            }
            next[i] = j;
        }
        j = 0;
        for (i, h) in haystack.iter().enumerate() {
            while j > 0 && *h != needle[j] {
                j = next[j - 1];
            }
            if haystack[i] == needle[j] {
                j += 1;
            }
            if j == n {
                return (i + 1 - j) as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::str_str("hello".to_string(), "ll".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(
            -1,
            Solution::str_str("aaaaa".to_string(), "bba".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(0, Solution::str_str("".to_string(), "".to_string()));
    }

    #[test]
    fn case4() {
        assert_eq!(-1, Solution::str_str("a".to_string(), "aa".to_string()));
    }

    #[test]
    fn case5() {
        assert_eq!(
            -1,
            Solution::str_str("mississippi".to_string(), "issipi".to_string())
        );
    }
}
