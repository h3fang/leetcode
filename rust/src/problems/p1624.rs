pub struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max = -1;
        let mut pos = [0; 26];
        for (i, ch) in s.as_bytes().iter().enumerate() {
            let j = (ch - b'a') as usize;
            if pos[j] > 0 {
                max = max.max(i as i32 - pos[j]);
            } else {
                pos[j] = i as i32 + 1;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(
            0,
            Solution::max_length_between_equal_characters("aa".to_string())
        );
    }

    #[test]
    fn case2() {
        assert_eq!(
            0,
            Solution::max_length_between_equal_characters("aa".to_string())
        );
    }
}
