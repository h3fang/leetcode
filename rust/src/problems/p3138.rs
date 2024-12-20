pub struct Solution;

impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut f = [0; 26];
        for len in 1..=n / 2 {
            f[(s[len - 1] - b'a') as usize] += 1;
            if n % len != 0 {
                continue;
            }
            if s.chunks_exact(len).skip(1).all(|c| {
                let mut f1 = f;
                for &b in c {
                    f1[(b - b'a') as usize] -= 1;
                }
                f1.iter().all(|&x| x == 0)
            }) {
                return len as i32;
            }
        }
        n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(2, Solution::min_anagram_length("abba".to_string()));
    }

    #[test]
    fn case2() {
        assert_eq!(4, Solution::min_anagram_length("cdef".to_string()));
    }
}
