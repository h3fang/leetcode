pub struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let s = s.as_bytes();
        let mut m = [[usize::MAX, 0]; 26];
        for (i, &c) in s.iter().enumerate() {
            let j = (c - b'a') as usize;
            if m[j][0] == usize::MAX {
                m[j][0] = i;
            }
            m[j][1] = i;
        }
        let mut result = 0;
        for [l, r] in m {
            if l >= r {
                continue;
            }
            let mask = s[l + 1..r]
                .iter()
                .fold(0u32, |acc, &e| acc | 1 << (e - b'a'));
            result += mask.count_ones();
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            3,
            Solution::count_palindromic_subsequence("aabca".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::count_palindromic_subsequence("adc".to_string())
        );
    }

    #[test]
    fn case3() {
        assert_eq!(
            4,
            Solution::count_palindromic_subsequence("bbcbaba".to_string())
        );
    }
}
