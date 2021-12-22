pub struct Solution;

impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        fn is_palindrome(w: &str) -> bool {
            let mut i = 0;
            let mut j = w.len() - 1;
            let w = w.as_bytes();
            while i < j {
                if w[i] != w[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        }
        for w in words {
            if is_palindrome(&w) {
                return w;
            }
        }
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let words = ["abc", "car", "ada", "racecar", "cool"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!("ada", Solution::first_palindrome(words));
    }

    #[test]
    fn case2() {
        let words = ["def", "ghi"];
        let words = words.iter().map(|w| w.to_string()).collect();
        assert_eq!("", Solution::first_palindrome(words));
    }
}
