pub struct Solution;

impl Solution {
    pub fn permutation(s: String) -> Vec<String> {
        fn next_perm(bytes: &mut [u8]) -> bool {
            if bytes.len() < 2 {
                return false;
            }
            let mut i = bytes.len() - 2;
            while bytes[i] >= bytes[i + 1] {
                if i == 0 {
                    return false;
                }
                i -= 1;
            }
            let mut j = bytes.len() - 1;
            while bytes[j] <= bytes[i] {
                j -= 1;
            }
            bytes.swap(i, j);
            bytes[i + 1..].reverse();
            true
        }
        let mut bytes = s.into_bytes();
        bytes.sort_unstable();
        let mut result = vec![unsafe { String::from_utf8_unchecked(bytes.to_vec()) }];
        while next_perm(&mut bytes) {
            unsafe {
                result.push(String::from_utf8_unchecked(bytes.to_vec()));
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
        let mut result = Solution::permutation("abc".to_string());
        result.sort_unstable();
        let expected = ["abc", "acb", "bac", "bca", "cab", "cba"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }

    #[test]
    fn case2() {
        let mut result = Solution::permutation("aab".to_string());
        result.sort_unstable();
        let expected = ["aba", "aab", "baa"];
        let mut expected = expected.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        expected.sort_unstable();
        assert_eq!(expected, result);
    }
}
