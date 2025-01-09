pub struct Solution;

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let mut f = [0; 26];
        for &b in word2.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        let (mut i, mut result) = (0, 0);
        let mut mismatch = f.iter().filter(|&&x| x != 0).count();
        let w = word1.as_bytes();
        for &b in w {
            let k = (b - b'a') as usize;
            f[k] -= 1;
            if f[k] == 0 {
                mismatch -= 1;
            }
            while mismatch == 0 {
                let k = (w[i] - b'a') as usize;
                f[k] += 1;
                if f[k] == 1 {
                    mismatch += 1;
                }
                i += 1;
            }
            result += i;
        }
        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            1,
            Solution::valid_substring_count("bcca".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            10,
            Solution::valid_substring_count("abcabc".to_string(), "abc".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            0,
            Solution::valid_substring_count("abcabc".to_string(), "aaabc".to_string())
        );
    }

    #[test]
    fn case4() {
        assert_eq!(
            18,
            Solution::valid_substring_count("dcbdcdccb".to_string(), "cdd".to_string())
        );
    }
}
