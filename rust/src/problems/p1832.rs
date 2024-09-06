pub struct Solution;

impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut f = [0; 26];
        for b in sentence.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        f.iter().all(|&e| e >= 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert!(Solution::check_if_pangram(
            "thequickbrownfoxjumpsoverthelazydog".to_string()
        ));
    }

    #[test]
    fn case2() {
        assert!(!Solution::check_if_pangram("leetcode".to_string()));
    }
}
