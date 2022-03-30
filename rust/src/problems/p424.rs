pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut left = 0;
        let mut count = [0; 26];
        let mut max = 0;
        for (right, c) in s.chars().enumerate() {
            let i = (c as u8 - b'A') as usize;
            count[i] += 1;
            max = max.max(count[i]);
            if right - left + 1 > max + k as usize {
                let j = (s.as_bytes()[left] as u8 - b'A') as usize;
                count[j] -= 1;
                left += 1;
            }
        }
        (s.len() - left) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(4, Solution::character_replacement("ABAB".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::character_replacement("AABABBA".to_string(), 1));
    }
}
