pub struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut sig = [0; 26];
        for b in p.as_bytes() {
            sig[(b - b'a') as usize] += 1;
        }

        let n = p.len();
        let mut result = Vec::new();
        if s.len() < n {
            return result;
        }
        for i in 0..n {
            sig[(s.as_bytes()[i] - b'a') as usize] -= 1;
        }
        let mut mismatch = sig.iter().filter(|e| **e != 0).count();
        if mismatch == 0 {
            result.push(0);
        }
        for i in 1..=s.len() - n {
            match sig[(s.as_bytes()[i - 1] - b'a') as usize] {
                0 => mismatch += 1,
                -1 => mismatch -= 1,
                _ => {}
            }
            sig[(s.as_bytes()[i - 1] - b'a') as usize] += 1;
            match sig[(s.as_bytes()[i + n - 1] - b'a') as usize] {
                0 => mismatch += 1,
                1 => mismatch -= 1,
                _ => {}
            }
            sig[(s.as_bytes()[i + n - 1] - b'a') as usize] -= 1;
            if mismatch == 0 {
                result.push(i as i32);
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
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let mut expected = vec![0, 6];
        expected.sort_unstable();
        assert_eq!(expected, Solution::find_anagrams(s, p));
    }

    #[test]
    fn case2() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        let mut expected = vec![0, 1, 2];
        expected.sort_unstable();
        assert_eq!(expected, Solution::find_anagrams(s, p));
    }
}
