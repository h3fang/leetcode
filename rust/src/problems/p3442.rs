pub struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut f = [0; 26];
        for &b in s.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        let max = *f.iter().filter(|&&f| f % 2 == 1).max().unwrap();
        let min = *f.iter().filter(|&&f| f > 0 && f % 2 == 0).min().unwrap();
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(3, Solution::max_difference("aaaaabbc".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(1, Solution::max_difference("abcabcab".to_string()));
    }
}
