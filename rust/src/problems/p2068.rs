pub struct Solution;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut c = [0i32; 26];
        for b in word1.as_bytes() {
            c[(b - b'a') as usize] += 1;
        }

        for b in word2.as_bytes() {
            c[(b - b'a') as usize] -= 1;
        }

        c.iter().all(|e| e.abs() <= 3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let word1 = "aaaa".to_string();
        let word2 = "bccb".to_string();
        assert!(!Solution::check_almost_equivalent(word1, word2));
    }

    #[test]
    fn case2() {
        let word1 = "abcdeef".to_string();
        let word2 = "abaaacc".to_string();
        assert!(Solution::check_almost_equivalent(word1, word2));
    }

    #[test]
    fn case3() {
        let word1 = "cccddabba".to_string();
        let word2 = "babababab".to_string();
        assert!(Solution::check_almost_equivalent(word1, word2));
    }
}
