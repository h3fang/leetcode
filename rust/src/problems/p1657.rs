pub struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        let mut m1 = [0; 26];
        let mut m2 = [0; 26];
        for &c in word1.as_bytes() {
            m1[(c - b'a') as usize] += 1;
        }
        for &c in word2.as_bytes() {
            m2[(c - b'a') as usize] += 1;
        }
        for (&a, &b) in m1.iter().zip(&m2) {
            if (a > 0 && b == 0) || (a == 0 && b > 0) {
                return false;
            }
        }
        m1.sort_unstable();
        m2.sort_unstable();
        m1 == m2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            true,
            Solution::close_strings("abc".to_string(), "bca".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            false,
            Solution::close_strings("a".to_string(), "aa".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            true,
            Solution::close_strings("cabbba".to_string(), "abbccc".to_string())
        );
    }
}
