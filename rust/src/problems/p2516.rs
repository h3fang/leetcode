pub struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut f = [0; 3];
        for b in s.bytes() {
            match b {
                b'a' => f[0] += 1,
                b'b' => f[1] += 1,
                b'c' => f[2] += 1,
                _ => unreachable!(),
            }
        }
        if f.iter().any(|&x| x < k) {
            return -1;
        }
        let mut result = s.len();
        let mut l = 0;
        for (r, b) in s.bytes().enumerate() {
            match b {
                b'a' => f[0] -= 1,
                b'b' => f[1] -= 1,
                b'c' => f[2] -= 1,
                _ => unreachable!(),
            }
            while l < r && f.iter().any(|&x| x < k) {
                match s.as_bytes()[l] {
                    b'a' => f[0] += 1,
                    b'b' => f[1] += 1,
                    b'c' => f[2] += 1,
                    _ => unreachable!(),
                }
                l += 1;
            }
            if f.iter().all(|&x| x >= k) {
                result = result.min(s.len() - (r - l + 1));
            }
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(8, Solution::take_characters("aabaaaacaabc".to_string(), 2));
    }

    #[test]
    fn case2() {
        assert_eq!(-1, Solution::take_characters("a".to_string(), 1));
    }
}
