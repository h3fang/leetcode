pub struct Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut prefixes = [-1; 32];
        prefixes[0] = 0;
        let mut mask = 0;
        let mut result = 0;
        for (i, b) in s.bytes().enumerate() {
            match b {
                b'a' => mask ^= 1 << 0,
                b'e' => mask ^= 1 << 1,
                b'i' => mask ^= 1 << 2,
                b'o' => mask ^= 1 << 3,
                b'u' => mask ^= 1 << 4,
                _ => {}
            }
            if prefixes[mask] >= 0 {
                result = result.max(i as i32 + 1 - prefixes[mask]);
            } else {
                prefixes[mask] = i as i32 + 1;
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
        assert_eq!(
            13,
            Solution::find_the_longest_substring("eleetminicoworoep".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            5,
            Solution::find_the_longest_substring("leetcodeisgreat".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            6,
            Solution::find_the_longest_substring("bcbcbc".to_string())
        );
    }
}
