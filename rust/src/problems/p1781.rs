pub struct Solution;

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let s = s.as_bytes();
        let mut result = 0;
        for i in 0..s.len() {
            let mut f = [0; 26];
            f[(s[i] - b'a') as usize] = 1;
            let mut max = 1;
            for c in &s[i + 1..] {
                let k = (c - b'a') as usize;
                f[k] += 1;
                max = max.max(f[k]);
                let min = *f.iter().filter(|e| **e > 0).min().unwrap();
                result += max - min;
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
        assert_eq!(5, Solution::beauty_sum("aabcb".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(17, Solution::beauty_sum("aabcbaa".to_string()));
    }
}
