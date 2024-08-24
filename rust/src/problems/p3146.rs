pub struct Solution;

impl Solution {
    pub fn find_permutation_difference(s: String, t: String) -> i32 {
        let mut pos = [0; 26];
        for (i, b) in s.bytes().enumerate() {
            pos[(b - b'a') as usize] = i;
        }
        let mut result = 0;
        for (i, b) in t.bytes().enumerate() {
            let k = (b - b'a') as usize;
            result += pos[k].abs_diff(i) as i32;
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
            2,
            Solution::find_permutation_difference("abc".to_string(), "bac".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            12,
            Solution::find_permutation_difference("abcde".to_string(), "edbac".to_string())
        );
    }
}
